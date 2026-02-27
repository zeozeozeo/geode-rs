use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

mod bind;
mod main_;
mod modify;

fn convert_result(result: syn::Result<TokenStream2>) -> TokenStream {
    match result {
        Ok(ts) => ts.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn modify(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr: TokenStream2 = attr.into();
    let item: TokenStream2 = item.into();

    convert_result(modify::expand_modify(attr, item))
}

#[proc_macro_attribute]
pub fn geode_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: TokenStream2 = item.into();
    convert_result(main_::expand_geode_main(input))
}

#[proc_macro]
pub fn geode_bind(input: TokenStream) -> TokenStream {
    let input: TokenStream2 = input.into();
    convert_result(bind::expand_geode_bind(input))
}
