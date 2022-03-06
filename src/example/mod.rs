use crate::server::service::{ServerReaderWriter, Service};

pub trait HelloServer {
    const METHOD_NAMES: [&'static str; 1] = ["hello"];

    const METHODS: [for<'r> fn(&'r Self, ServerReaderWriter); 1] = [Self::hello];

    fn hello(&self, stream: ServerReaderWriter);
}

impl<S: HelloServer> Service for S {
    fn call_method(&self, fn_n: usize, stream: ServerReaderWriter) {
        Self::METHODS[fn_n](&self, stream);
    }

    fn method_names(&self) -> &'static [&'static str] {
        &Self::METHOD_NAMES
    }

    fn num_of_methods(&self) -> usize {
        Self::METHODS.len()
    }
}

pub struct HelloServerImpl {}

impl HelloServer for HelloServerImpl {
    fn hello(&self, stream: ServerReaderWriter) {
        todo!()
    }
}
