use std::path::Path;

use rocksdb::DB;
use crate::storage::Storage;

#[derive(Debug)]
pub struct RocksDbStorage(DB);

impl RocksDbStorage {
    fn new(path: impl AsRef<Path>) -> Self {
        Self(DB::open_default(path).unwrap())
    }
}

impl Storage for RocksDbStorage {
    fn get(&self,key: &str) -> Result<Option<bytes::Bytes>, Box<dyn std::error::Error>> {
        let value = self.0.get(key)?.unwrap();
        Ok(Some(value.into()))  
    }

    fn set(&self,key: &str,value: bytes::Bytes) -> Result<Option<bytes::Bytes>, Box<dyn std::error::Error>> {
        self.0.put(key,value.clone()).unwrap();
        Ok(Some(value))
    }
}