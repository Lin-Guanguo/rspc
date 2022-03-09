extern crate proc_macro;
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_attribute]
pub fn rspc_client(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("{:#?}", attr);
    item
}
