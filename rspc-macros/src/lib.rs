extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn rspc_client(attr: TokenStream, item: TokenStream) -> TokenStream {
    // let normal_rpc = vec![];
    // let stream_rpc = vec![];
    for ident in attr {
        println!("{:?}", ident);
        continue;
        match ident {
            TokenTree::Punct(_) => {}
            TokenTree::Group(_) => todo!(),
            TokenTree::Ident(_) => todo!(),
            TokenTree::Literal(_) => todo!(),
        }
    }
    item
}
