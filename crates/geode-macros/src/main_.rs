use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{ItemFn, Result};

pub fn expand_geode_main(input: TokenStream2) -> Result<TokenStream2> {
    let main_fn: ItemFn = syn::parse2(input)?;

    let fn_name = &main_fn.sig.ident;
    let fn_block = &main_fn.block;

    let expanded = quote! {
        #[unsafe(no_mangle)]
        pub extern "C" fn geodeImplicitEntry() {
            geode_rs::internal::init_mod();
            geode_rs::modify::flush_pending_hooks();
            #fn_name();
        }

        fn #fn_name() {
            #fn_block
        }
    };

    Ok(expanded)
}
