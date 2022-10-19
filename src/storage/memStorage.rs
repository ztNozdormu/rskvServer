use crate::storage::storage::Storage;
use bytes::Bytes;

#[derive(Clone,Debug)]
pub struct MemStorage {
  value: DashMap<String,Bytes>
}

impl MemStorage { 
    pub fn new(&self) -> Self {
        MemStorage { value: default::new() }
    }
}

impl Storage for MemStorage {
    
}
