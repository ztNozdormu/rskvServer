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
    
}