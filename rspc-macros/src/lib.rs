extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    self,
    parse::{Parse, ParseStream},
    parse_macro_input, DeriveInput, Ident, Token,
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
    let normal_n = normal.len() as u32;
    let normal_id = 0..normal_n;
    let stream = attr.stream.iter();
    let stream_n = stream.len() as u32;
    let stream_id = (0..stream_n).map(|x| x + normal_n);
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

    println!("{}", ret);

    let mut item = item;
    let t: TokenStream = ret.into();
    item.extend(t);
    item
}

#[proc_macro_attribute]
pub fn rspc_server(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as RpcMethods);
    println!("{:?}", attr);

    let server: DeriveInput = syn::parse(item.clone()).unwrap();
    let generics = server.generics;
    let name = server.ident;
    let name_literal = name.to_string();

    let normal = attr.normal.iter();
    let normal_n = normal.len() as u32;
    let normal_id = 0..normal_n;
    let normal_literal = normal.map(|x| x.to_string());
    let stream = attr.stream.iter();
    let stream_n = stream.len() as u32;
    let stream_id = (0..stream_n).map(|x| x + normal_n);
    let stream_literal = stream.map(|x| x.to_string());

    let ret = quote! {
        #[async_trait::async_trait(?Send)]
        impl #generics rspc::server::Service for #name #generics {
            async fn call_method(
                &self,
                fn_n: u32,
                mut stream: rspc::server::ServerReaderWriter,
            ) -> Result<(), rspc::server::ServerError> {
                if fn_n < #normal_n {
                    if let Some(request) = stream.read().await {
                        let reply = match fn_n {
                            #(
                                #normal_id => self.hello(request).await,
                            )*

                            _ => return Err(rspc::server::ServerError::NormalRpcMethodError()),
                        };

                        stream.write(reply.0, reply.1).await?;
                        Ok(())
                    } else {
                        Err(rspc::server::ServerError::NormalRpcMethodError())
                    }
                } else {
                    match fn_n {
                        #(
                            #stream_id => self.hello_stream(stream).await,
                        )*

                        _ => Err(rspc::server::ServerError::StreamRpcMethodError()),
                    }
                }
            }

            fn service_name(&self) -> &'static str {
                #name_literal
            }

            fn methods_name(&self) -> &'static [&'static str] {
                &[
                    #(#normal_literal,)*
                    #(#stream_literal,)*
                ]
            }

            fn methods_len(&self) -> usize {
                (#normal_n + #stream_n) as usize
            }
        }
    };

    println!("{}", ret);

    let mut item = item;
    let t: TokenStream = ret.into();
    item.extend(t);
    item
}
