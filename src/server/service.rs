use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use bytes::Bytes;

pub type ServiceFn = fn(Bytes) -> (u32, Bytes);

#[derive(Clone)]
pub struct ServiceTable {
    table: Arc<RwLock<HashMap<u32, ServiceFn>>>,
}

impl ServiceTable {
    pub fn new() -> Self {
        Self {
            table: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn insert(&mut self, method_id: u32, service_fn: ServiceFn) -> Option<ServiceFn> {
        let mut map = self.table.write().unwrap();
        map.insert(method_id, service_fn)
    }

    pub fn get(&self, method_id: u32) -> Option<ServiceFn> {
        let map = self.table.read().unwrap();
        map.get(&method_id).copied()
    }
}
