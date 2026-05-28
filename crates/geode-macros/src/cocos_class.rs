use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Fields, ItemStruct, Path, Result, Visibility};

struct CocosClassArgs {
    base: Path,
}

impl Parse for CocosClassArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            base: input.parse()?,
        })
    }
}

pub fn expand_cocos_class(args: TokenStream2, item: TokenStream2) -> Result<TokenStream2> {
    let args: CocosClassArgs = syn::parse2(args)?;
    let struct_item: ItemStruct = syn::parse2(item)?;

    let attrs = &struct_item.attrs;
    let vis = &struct_item.vis;
    let name = &struct_item.ident;
    let generics = &struct_item.generics;
    let base = &args.base;

    if !generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            generics,
            "inherit does not support generic Cocos subclasses yet",
        ));
    }

    let named = match &struct_item.fields {
        Fields::Named(named) => named,
        _ => {
            return Err(syn::Error::new_spanned(
                &struct_item.fields,
                "inherit requires a struct with named fields",
            ));
        }
    };

    let fields = named.named.iter();
    let defaults: Vec<_> = named
        .named
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().expect("named field");
            quote! {
                ::std::ptr::addr_of_mut!((*this.as_ptr()).#ident)
                    .write(::std::default::Default::default());
            }
        })
        .collect();

    let base_vis = match vis {
        Visibility::Public(_) => quote!(pub),
        _ => quote!(),
    };

    let expanded = quote! {
        #(#attrs)*
        #[repr(C)]
        #vis struct #name #generics {
            #base_vis base: #base,
            #(#fields),*
        }

        impl #name {
            pub fn try_alloc_uninit() -> Option<::std::ptr::NonNull<Self>> {
                ::geode_rs::inherit::try_alloc_cocos_object::<Self>()
            }

            pub unsafe fn alloc_uninit() -> *mut Self {
                unsafe { ::geode_rs::inherit::alloc_cocos_object::<Self>() }
            }

            pub fn init_fields(this: ::std::ptr::NonNull<Self>) {
                unsafe {
                    #(#defaults)*
                }
            }

            pub unsafe fn init_default_fields(this: *mut Self) {
                if let Some(this) = ::std::ptr::NonNull::new(this) {
                    Self::init_fields(this);
                }
            }

            pub fn base_ptr(this: ::std::ptr::NonNull<Self>) -> ::std::ptr::NonNull<#base> {
                this.cast()
            }

            pub fn with_mut<R>(
                mut this: ::std::ptr::NonNull<Self>,
                f: impl FnOnce(&mut Self) -> R,
            ) -> R {
                unsafe { f(this.as_mut()) }
            }

            pub fn with_base_mut<R>(
                mut this: ::std::ptr::NonNull<Self>,
                f: impl FnOnce(&mut #base) -> R,
            ) -> R {
                unsafe { f(&mut this.as_mut().base) }
            }

            pub unsafe fn as_base_ptr(this: *mut Self) -> *mut #base {
                this.cast()
            }

            pub fn cc_object_ptr(
                this: ::std::ptr::NonNull<Self>,
            ) -> ::std::ptr::NonNull<::geode_rs::classes::CCObject> {
                this.cast()
            }

            pub unsafe fn as_cc_object_ptr(this: *mut Self) -> *mut ::geode_rs::classes::CCObject {
                this.cast()
            }

            pub fn autorelease(this: ::std::ptr::NonNull<Self>) -> ::std::ptr::NonNull<Self> {
                ::geode_rs::inherit::try_autorelease_as_ccobject(this)
            }

            pub unsafe fn delete_uninit(this: ::std::ptr::NonNull<Self>) {
                unsafe {
                    ::geode_rs::inherit::cxx_operator_delete(this.as_ptr().cast());
                }
            }

            pub fn base(&self) -> &#base {
                &self.base
            }

            pub fn base_mut(&mut self) -> &mut #base {
                &mut self.base
            }
        }

        impl ::std::ops::Deref for #name {
            type Target = #base;

            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }

        impl ::std::ops::DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }
        }
    };

    Ok(expanded)
}
