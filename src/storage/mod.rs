use std::error::Error;
use bytes::Bytes;

pub mod mem_storage;
mod rocks_db_storage;

pub trait Storage {
    fn get(&self,key: &str) -> Result<Option<Bytes>, Box<dyn Error>>;
    fn set(&self,key: &str,value: Bytes) -> Result<Option<Bytes>, Box<dyn Error>>;
}