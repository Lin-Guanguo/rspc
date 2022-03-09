extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use syn::{
    self,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    DeriveInput, Ident, Token,
};

#[derive(Debug)]
struct RpcMethods {
    normal: Vec<Ident>,
    stream: Vec<Ident>,
}

impl Parse for RpcMethods {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut normal = vec![];
        let mut stream = vec![];
        loop {
            let first = input.parse::<Ident>()?;
            if first.to_string() == "stream" {
                let second = input.parse::<Ident>()?;
                stream.push(second);
            } else {
                normal.push(first);
            }
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(Self { normal, stream })
    }
}

#[proc_macro_attribute]
pub fn rspc_client(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as RpcMethods);
    println!("{:?}", attr);

    let client: DeriveInput = syn::parse(item.clone()).unwrap();
    let generics = client.generics;
    let name = client.ident;

    let normal = attr.normal.iter();
    let normal_id = 0..normal.len() as u32;
    let stream = attr.stream.iter();
    let stream_id = normal.len() as u32..normal.len() as u32 + stream.len() as u32;
    let stream_impl = attr.stream.iter().map(|i| format_ident!("{}_impl", i));
    let ret = quote! {
        impl #generics #name #generics {
            #(
                pub async fn #normal(&self, request: bytes::Bytes) -> Result<(u32, bytes::Bytes), rspc::client::ClientError> {
                    let id = rspc::client::ClientStub::first_method_id(self);
                    let mut rw: rspc::client::ClientReaderWriter =
                        rspc::client::ClientStub::channel(self).call_method(id + #normal_id);

                    rw.write_last(request).await?;
                    Ok(rw.read().await.unwrap())
                }
            )*

            #(
                pub async fn #stream(&self) {
                    let id = rspc::client::ClientStub::first_method_id(self);
                    let rw: rspc::client::ClientReaderWriter =
                        rspc::client::ClientStub::channel(self).call_method(id + #stream_id);
                    self.#stream_impl(rw).await
                }
            )*
        }
    };

    println!("{:#?}", ret);

    let mut item = item;
    let t: TokenStream = ret.into();
    item.extend(t);
    item
}
