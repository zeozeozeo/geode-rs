use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{FnArg, Ident, Item, ItemImpl, ItemStruct, Result, Type, TypeReference};

pub fn expand_modify(class_name: Ident, item: TokenStream2) -> Result<TokenStream2> {
    let parsed: Item = syn::parse2(item.clone())?;

    match parsed {
        Item::Struct(struct_item) => expand_modify_struct(class_name, struct_item),
        Item::Impl(impl_item) => expand_modify_impl(class_name, impl_item),
        _ => Err(syn::Error::new_spanned(
            parsed,
            "modify attribute can only be applied to structs or impl blocks",
        )),
    }
}

fn expand_modify_struct(class_name: Ident, struct_item: ItemStruct) -> Result<TokenStream2> {
    let struct_name = &struct_item.ident;
    let struct_name_str = struct_name.to_string();
    let storage_ident = format_ident!("__{}_STORAGE", struct_name_str.to_uppercase());

    let field_names: Vec<_> = struct_item
        .fields
        .iter()
        .filter_map(|f| f.ident.clone())
        .collect();

    let expanded = quote! {
        #struct_item

        #[allow(non_upper_case_globals)]
        static #storage_ident: ::geode_rs::modify::ModifyStorage<#struct_name> =
            ::geode_rs::modify::ModifyStorage::new();

        impl #struct_name {
            pub fn get(this: *mut ::geode_rs::classes::#class_name) -> Option<&'static mut Self> {
                #storage_ident.get(this as usize)
            }

            pub fn get_or_default(this: *mut ::geode_rs::classes::#class_name) -> &'static mut Self {
                #storage_ident.get_or_default(this as usize, || Self {
                    #(#field_names: Default::default()),*
                })
            }
        }
    };

    Ok(expanded)
}

fn expand_modify_impl(class_name: Ident, impl_block: ItemImpl) -> Result<TokenStream2> {
    let struct_name = if let syn::Type::Path(path) = &*impl_block.self_ty {
        path.path
            .segments
            .last()
            .map(|s| s.ident.clone())
            .ok_or_else(|| syn::Error::new_spanned(&impl_block.self_ty, "invalid type"))?
    } else {
        return Err(syn::Error::new_spanned(
            &impl_block.self_ty,
            "expected a type path",
        ));
    };

    let mut hook_registrations: Vec<TokenStream2> = Vec::new();
    let mut detour_functions: Vec<TokenStream2> = Vec::new();

    for item in &impl_block.items {
        if let syn::ImplItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            let method_name_str = method_name.to_string();

            let has_self_param = method
                .sig
                .inputs
                .first()
                .is_some_and(|arg| matches!(arg, FnArg::Receiver(_)));

            let addr_const = format_ident!("{}_ADDR", method_name_str.to_uppercase());

            let convention = quote!(::geode_rs::CallingConvention::Default);

            let detour_func_name = format_ident!(
                "__detour_{}_{}",
                to_snake_case(&class_name.to_string()),
                to_snake_case(&method_name_str)
            );

            let hook_name = format!("{}::{}", class_name, method_name_str);

            let output = &method.sig.output;
            let block = &method.block;

            let (detour_params, call_args) = build_detour_params_and_call_args(
                &method.sig.inputs,
                &struct_name,
                &class_name,
                has_self_param,
            );

            let detour_func = if has_self_param {
                quote! {
                    #[allow(clippy::not_unsafe_ptr_arg_deref)]
                    pub extern "C" fn #detour_func_name(#detour_params) #output {
                        #struct_name::#method_name(#call_args)
                    }
                }
            } else {
                quote! {
                    #[allow(clippy::not_unsafe_ptr_arg_deref)]
                    pub extern "C" fn #detour_func_name(#detour_params) #output {
                        #block
                    }
                }
            };

            detour_functions.push(detour_func);

            let class_name_inner = class_name.clone();
            let hook_registration = quote! {
                unsafe {
                    let addr = ::geode_rs::classes::#class_name_inner::#addr_const();
                    if addr == 0 {
                        #[cfg(target_os = "android")]
                        ::geode_rs::loader::android_log(
                            concat!("WARN: address for ", #hook_name, " resolved to 0, skipping hook\0").as_bytes()
                        );
                        #[cfg(not(target_os = "android"))]
                        eprintln!("[geode-rs] WARN: address for {} is 0, skipping hook", #hook_name);
                    } else {
                        let detour = #detour_func_name as *mut ::std::ffi::c_void;
                        let _ = ::geode_rs::modify::register_hook(addr, detour, #hook_name, #convention);
                    }
                }
            };
            hook_registrations.push(hook_registration);
        }
    }

    let hooks_static_name = format_ident!(
        "__MODIFY_HOOKS_{}",
        to_snake_case(&struct_name.to_string()).to_uppercase()
    );

    let expanded = quote! {
        #impl_block

        #(#detour_functions)*

        #[used]
        #[::geode_rs::ctor::ctor(crate_path = ::geode_rs::ctor)]
        static #hooks_static_name: () = {
            #(#hook_registrations)*
        };
    };

    Ok(expanded)
}

fn build_detour_params_and_call_args(
    inputs: &syn::punctuated::Punctuated<FnArg, syn::Token![,]>,
    struct_name: &Ident,
    class_name: &Ident,
    has_self_param: bool,
) -> (TokenStream2, TokenStream2) {
    let mut detour_params: Vec<TokenStream2> = Vec::new();
    let mut call_args: Vec<TokenStream2> = Vec::new();

    detour_params.push(quote!(this: *mut ::geode_rs::classes::#class_name));

    if has_self_param {
        call_args.push(quote!(#struct_name::get_or_default(this)));
    }

    let mut found_this = false;
    for arg in inputs.iter() {
        if let FnArg::Typed(pat_type) = arg {
            let pat = &pat_type.pat;
            let ty = &pat_type.ty;

            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat
                && pat_ident.ident == "this"
            {
                found_this = true;
                if is_mut_ref_type(ty) {
                    call_args.push(quote!(unsafe { &mut *this }));
                } else if is_const_ref_type(ty) {
                    call_args.push(quote!(unsafe { &*this }));
                } else {
                    call_args.push(quote!(this));
                }
                continue;
            }

            if is_mut_ref_type(ty) {
                let inner_ty = extract_ref_inner_type(ty);
                detour_params.push(quote!(#pat: *mut #inner_ty));
                call_args.push(quote!(unsafe { &mut *#pat }));
            } else if is_const_ref_type(ty) {
                let inner_ty = extract_ref_inner_type(ty);
                detour_params.push(quote!(#pat: *const #inner_ty));
                call_args.push(quote!(unsafe { &*#pat }));
            } else {
                detour_params.push(quote!(#pat: #ty));
                call_args.push(quote!(#pat));
            }
        }
    }

    if !found_this {
        call_args.push(quote!(this));
    }

    let detour_params_stream = quote!(#(#detour_params),*);
    let call_args_stream = quote!(#(#call_args),*);

    (detour_params_stream, call_args_stream)
}

fn is_mut_ref_type(ty: &Type) -> bool {
    if let Type::Reference(TypeReference {
        mutability: Some(_),
        ..
    }) = ty
    {
        return true;
    }
    false
}

fn is_const_ref_type(ty: &Type) -> bool {
    if let Type::Reference(TypeReference {
        mutability: None, ..
    }) = ty
    {
        return true;
    }
    false
}

fn extract_ref_inner_type(ty: &Type) -> &Type {
    if let Type::Reference(TypeReference { elem, .. }) = ty {
        elem
    } else {
        ty
    }
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    let mut prev_was_lower = false;
    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            let next_is_lower = chars.peek().is_some_and(|n| n.is_lowercase());
            if !result.is_empty() && (prev_was_lower || next_is_lower) {
                result.push('_');
            }
            result.extend(c.to_lowercase());
            prev_was_lower = false;
        } else {
            result.push(c);
            prev_was_lower = c.is_lowercase();
        }
    }
    result
}
