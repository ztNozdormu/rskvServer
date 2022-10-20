use std::error::Error;
use bytes::Bytes;
use dashmap::DashMap;

use crate::Storage;

#[derive(Clone,Debug,Default)]
pub struct MemStorage {
  map: DashMap<String,Bytes>
}

impl MemStorage { 
    #[warn(dead_code)]
    pub fn new() -> Self {
        MemStorage { map: Default::default() }
    }
}

impl Storage for MemStorage {
    fn get(&self,key: &str) -> Result<Option<Bytes>, Box<dyn Error>> {
      Ok(self.map.get(key).map(|value|value.clone()))
}

    fn set(&self,key: &str,value: Bytes) -> Result<Option<Bytes>, Box<dyn Error>> {
        self.map.insert(key.to_string(), value.clone());
        Ok(Some(value))
    }
}