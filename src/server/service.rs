use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use bytes::Bytes;

pub type ServiceFn = fn(Bytes) -> (u32, Bytes);

pub type ServiceTable = Arc<RwLock<HashMap<u32, ServiceFn>>>;
