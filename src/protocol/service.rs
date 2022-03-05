use bytes::Bytes;

pub trait Service {
    fn call_method(&self, fn_n: usize, reqeust: Bytes) -> (u32, Bytes);

    fn method_names(&self) -> &'static [&'static str];

    fn num_of_methods(&self) -> usize;
}
