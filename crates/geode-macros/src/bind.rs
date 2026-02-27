use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    Ident, LitByteStr, LitStr, Result, Token, Type, Visibility,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
};

struct PlatformEntry {
    platform: Ident,
    symbol_bytes: Vec<u8>,
    symbol_span: proc_macro2::Span,
}

impl Parse for PlatformEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let platform: Ident = input.parse()?;
        input.parse::<Token![:]>()?;

        let (symbol_bytes, symbol_span) = if input.peek(syn::LitByteStr) {
            let lit: LitByteStr = input.parse()?;
            let mut bytes = lit.value();
            while bytes.last() == Some(&0) {
                bytes.pop();
            }
            bytes.push(0);
            (bytes, lit.span())
        } else {
            let lit: LitStr = input.parse()?;
            let mut bytes = lit.value().into_bytes();
            bytes.push(0);
            (bytes, lit.span())
        };

        Ok(Self {
            platform,
            symbol_bytes,
            symbol_span,
        })
    }
}

struct FnArg {
    name: Ident,
    ty: Type,
}

impl Parse for FnArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        Ok(Self { name, ty })
    }
}

#[derive(Clone)]
enum ReturnKind {
    Plain(Type),
    Sret(Type),
    MethodSret(Type),
    Void,
}

struct BindingDecl {
    attrs: Vec<syn::Attribute>,
    vis: Visibility,
    unsafety: Option<Token![unsafe]>,
    fn_name: Ident,
    args: Vec<FnArg>,
    ret: ReturnKind,
    platforms: Vec<PlatformEntry>,
}

impl Parse for BindingDecl {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let unsafety: Option<Token![unsafe]> = input.parse()?;
        input.parse::<Token![fn]>()?;
        let fn_name: Ident = input.parse()?;

        let args_content;
        syn::parenthesized!(args_content in input);
        let args_punct: Punctuated<FnArg, Comma> =
            args_content.parse_terminated(FnArg::parse, Token![,])?;
        let args: Vec<FnArg> = args_punct.into_iter().collect();

        // sret/method_sret?
        let ret = if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            if input.peek(Ident) {
                let lookahead = input.fork();
                let maybe_kw: Ident = lookahead.parse()?;
                if maybe_kw == "sret" {
                    input.parse::<Ident>()?;
                    let ty: Type = input.parse()?;
                    ReturnKind::Sret(ty)
                } else if maybe_kw == "method_sret" {
                    input.parse::<Ident>()?;
                    let ty: Type = input.parse()?;
                    ReturnKind::MethodSret(ty)
                } else {
                    let ty: Type = input.parse()?;
                    ReturnKind::Plain(ty)
                }
            } else {
                let ty: Type = input.parse()?;
                ReturnKind::Plain(ty)
            }
        } else {
            ReturnKind::Void
        };

        let body_content;
        syn::braced!(body_content in input);
        let entries: Punctuated<PlatformEntry, Comma> =
            body_content.parse_terminated(PlatformEntry::parse, Token![,])?;
        let platforms: Vec<PlatformEntry> = entries.into_iter().collect();

        Ok(Self {
            attrs,
            vis,
            unsafety,
            fn_name,
            args,
            ret,
            platforms,
        })
    }
}

struct BindInput {
    decls: Vec<BindingDecl>,
}

impl Parse for BindInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut decls = Vec::new();
        while !input.is_empty() {
            decls.push(input.parse()?);
        }
        Ok(Self { decls })
    }
}

pub fn expand_geode_bind(input: TokenStream2) -> Result<TokenStream2> {
    let BindInput { decls } = syn::parse2(input)?;
    let mut output = TokenStream2::new();
    for decl in decls {
        output.extend(expand_one(decl)?);
    }
    Ok(output)
}

fn expand_one(decl: BindingDecl) -> Result<TokenStream2> {
    match &decl.ret {
        ReturnKind::Sret(_) | ReturnKind::MethodSret(_) => expand_sret(decl),
        ReturnKind::Plain(_) | ReturnKind::Void => expand_plain(decl),
    }
}

fn expand_plain(decl: BindingDecl) -> Result<TokenStream2> {
    let BindingDecl {
        attrs,
        vis,
        unsafety,
        fn_name,
        args,
        ret,
        platforms,
    } = decl;

    let ret_ty = match &ret {
        ReturnKind::Plain(ty) => quote!(#ty),
        ReturnKind::Void | ReturnKind::Sret(_) | ReturnKind::MethodSret(_) => quote!(()),
    };

    let static_name = format_ident!("__GEODE_BIND_{}", fn_name.to_string().to_uppercase());
    let arg_names: Vec<&Ident> = args.iter().map(|a| &a.name).collect();
    let arg_types: Vec<&Type> = args.iter().map(|a| &a.ty).collect();

    let mut platform_arms = TokenStream2::new();
    for entry in &platforms {
        let cfg = platform_key_to_cfg(&entry.platform)?;
        let sym_lit = LitByteStr::new(&entry.symbol_bytes, entry.symbol_span);
        let resolver = platform_key_to_resolver(&entry.platform, &sym_lit);
        let abi = platform_key_to_abi(&entry.platform);

        platform_arms.extend(quote! {
            #[cfg(#cfg)]
            {
                static #static_name: ::std::sync::OnceLock<::std::option::Option<usize>>
                    = ::std::sync::OnceLock::new();
                let addr = *#static_name.get_or_init(|| { #resolver });
                if let ::std::option::Option::Some(addr) = addr {
                    let func: unsafe extern #abi fn(#(#arg_types),*) -> #ret_ty =
                        unsafe { ::std::mem::transmute(addr) };
                    return ::std::option::Option::Some(unsafe { func(#(#arg_names),*) });
                } else {
                    return ::std::option::Option::None;
                }
            }
        });
    }

    Ok(quote! {
        #(#attrs)*
        #[allow(non_upper_case_globals, dead_code, unsafe_op_in_unsafe_fn)]
        #vis #unsafety fn #fn_name(#(#arg_names: #arg_types),*) -> ::std::option::Option<#ret_ty> {
            #platform_arms
            #[allow(unreachable_code)]
            ::std::option::Option::None
        }
    })
}

fn expand_sret(decl: BindingDecl) -> Result<TokenStream2> {
    let BindingDecl {
        attrs,
        vis,
        unsafety,
        fn_name,
        args,
        ret,
        platforms,
    } = decl;

    let (out_ty, is_method) = match &ret {
        ReturnKind::Sret(ty) => (ty.clone(), false),
        ReturnKind::MethodSret(ty) => (ty.clone(), true),
        _ => unreachable!(),
    };

    let static_name = format_ident!("__GEODE_BIND_{}", fn_name.to_string().to_uppercase());
    let arg_names: Vec<&Ident> = args.iter().map(|a| &a.name).collect();
    let arg_types: Vec<&Type> = args.iter().map(|a| &a.ty).collect();

    let mut platform_arms = TokenStream2::new();

    for entry in &platforms {
        let platform_str = entry.platform.to_string();
        let cfg = platform_key_to_cfg(&entry.platform)?;
        let sym_lit = LitByteStr::new(&entry.symbol_bytes, entry.symbol_span);
        let resolver = platform_key_to_resolver(&entry.platform, &sym_lit);

        let is_aarch64 = matches!(
            platform_str.as_str(),
            "android64" | "mac_arm" | "m1" | "ios" | "ios_arm"
        );

        let addr_resolve = quote! {
            static #static_name: ::std::sync::OnceLock<::std::option::Option<usize>>
                = ::std::sync::OnceLock::new();
            let addr = match *#static_name.get_or_init(|| { #resolver }) {
                ::std::option::Option::Some(a) => a,
                ::std::option::Option::None => return ::std::option::Option::None,
            };
        };

        let abi = platform_key_to_abi(&entry.platform);
        let is_windows = matches!(entry.platform.to_string().as_str(), "win" | "windows");
        let call = if is_aarch64 {
            emit_aarch64_sret_call(&arg_names)
        } else if is_windows && is_method {
            emit_win64_method_sret_call(&arg_names, &arg_types, &out_ty)
        } else {
            emit_hidden_ptr_sret_call(&arg_names, &arg_types, &out_ty, &abi)
        };

        platform_arms.extend(quote! {
            #[cfg(#cfg)]
            {
                #addr_resolve
                let mut out = ::std::mem::MaybeUninit::<#out_ty>::uninit();
                #call
                return ::std::option::Option::Some(unsafe { out.assume_init() });
            }
        });
    }

    Ok(quote! {
        #(#attrs)*
        #[allow(non_upper_case_globals, dead_code, unsafe_op_in_unsafe_fn)]
        #vis #unsafety fn #fn_name(#(#arg_names: #arg_types),*) -> ::std::option::Option<#out_ty> {
            #platform_arms
            #[allow(unreachable_code)]
            ::std::option::Option::None
        }
    })
}

fn emit_aarch64_sret_call(arg_names: &[&Ident]) -> TokenStream2 {
    let nargs = arg_names.len();
    if nargs > 7 {
        return quote! {
            ::std::compile_error!(
                "geode_bind! sret on aarch64: too many arguments (max 7 + implicit x8 sret)"
            );
        };
    }

    let regs = ["x0", "x1", "x2", "x3", "x4", "x5", "x6"];
    let mut in_regs = TokenStream2::new();

    for (i, name) in arg_names.iter().enumerate() {
        let reg = regs[i];
        in_regs.extend(quote! {
            in(#reg) {
                let mut __tmp = 0usize;
                let __size = ::std::mem::size_of_val(&#name);
                let __copy_len = if __size > 8 { 8 } else { __size };
                unsafe {
                    ::std::ptr::copy_nonoverlapping(
                        &#name as *const _ as *const u8,
                        &mut __tmp as *mut _ as *mut u8,
                        __copy_len,
                    );
                }
                __tmp
            },
        });
    }

    quote! {
        unsafe {
            ::std::arch::asm!(
                "blr {fn_ptr}",
                fn_ptr = in(reg) addr,
                in("x8") out.as_mut_ptr() as usize,
                #in_regs
                clobber_abi("C"),
            );
        }
    }
}

fn emit_win64_method_sret_call(
    arg_names: &[&Ident],
    arg_types: &[&Type],
    out_ty: &Type,
) -> TokenStream2 {
    if arg_names.is_empty() {
        return quote! {
            ::std::compile_error!(
                "geode_bind! method_sret: instance method must have at least one argument (self/this)"
            );
        };
    }
    let this_name = arg_names[0];
    let this_type = arg_types[0];
    let rest_names = &arg_names[1..];
    let rest_types = &arg_types[1..];

    quote! {
        unsafe {
            let func: unsafe extern "system" fn(#this_type, *mut #out_ty, #(#rest_types),*) -> () =
                ::std::mem::transmute(addr);
            func(#this_name, out.as_mut_ptr(), #(#rest_names),*);
        }
    }
}

fn emit_hidden_ptr_sret_call(
    arg_names: &[&Ident],
    arg_types: &[&Type],
    out_ty: &Type,
    abi: &TokenStream2,
) -> TokenStream2 {
    quote! {
        unsafe {
            let func: unsafe extern #abi fn(*mut #out_ty, #(#arg_types),*) -> () =
                ::std::mem::transmute(addr);
            func(out.as_mut_ptr(), #(#arg_names),*);
        }
    }
}

fn platform_key_to_abi(ident: &Ident) -> TokenStream2 {
    match ident.to_string().as_str() {
        "win" | "windows" => quote!("system"),
        _ => quote!("C"),
    }
}

fn platform_key_to_cfg(ident: &Ident) -> Result<TokenStream2> {
    let key = ident.to_string();
    let ts = match key.as_str() {
        "win" | "windows" => quote!(target_os = "windows"),
        "mac" | "macos" => quote!(target_os = "macos"),
        "mac_intel" | "imac" => quote!(all(target_os = "macos", target_arch = "x86_64")),
        "mac_arm" | "m1" => quote!(all(target_os = "macos", target_arch = "aarch64")),
        "ios" => quote!(target_os = "ios"),
        "ios_arm" => quote!(all(target_os = "ios", target_arch = "aarch64")),
        "android32" => quote!(all(target_os = "android", target_arch = "arm")),
        "android64" => quote!(all(target_os = "android", target_arch = "aarch64")),
        "android" => quote!(target_os = "android"),
        other => {
            return Err(syn::Error::new(
                ident.span(),
                format!(
                    "unknown platform `{other}`. \
                 Expected one of: win, mac, mac_intel (imac), mac_arm (m1), \
                 ios, android32, android64, android"
                ),
            ));
        }
    };
    Ok(ts)
}

fn platform_key_to_resolver(ident: &Ident, sym: &LitByteStr) -> TokenStream2 {
    let key = ident.to_string();
    match key.as_str() {
        "win" | "windows" => quote! {
            unsafe {
                let base = crate::base::get_geode();
                crate::base::get_proc_address(base, #sym)
            }
        },
        "mac" | "macos" | "mac_intel" | "imac" | "mac_arm" | "m1" | "ios" | "ios_arm" => quote! {
            unsafe { crate::base::dylib_resolve_sym(#sym) }
        },
        "android32" | "android64" | "android" => quote! {
            unsafe { crate::base::android_dlsym_geode(#sym) }
        },
        _ => quote!(::std::option::Option::None),
    }
}
