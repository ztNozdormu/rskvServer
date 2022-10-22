mod cmd_service;

use std::sync::Arc;

use crate::CmdRequest;
use crate::CmdResponse;
use crate::Storage;
use crate::cmd_request::ReqData;
use crate::storage::rocks_db_storage::RocksDbStorage;

// 定义命令执行公共行为
pub trait CmdService {
    // 命令执行方法  &impl Storage 与&dyn Storage的区别 ？ TODO
    fn excute(self,stor: &dyn Storage) -> CmdResponse;
}
// 默认存储为RocksDbStorage
pub struct Service<S = RocksDbStorage> {
    store_srv: Arc<StorageService<S>>,
}

// 需要在多线程中clone
pub struct StorageService<Store> {
    store: Store,
} 

impl <Store: Storage> Service<Store> {
    
    pub fn new(store: Store) -> Self {
        Self { store_srv: Arc::new(StorageService{store}) }
    }

    pub async fn excute(&self,cmd_req: CmdRequest) -> CmdResponse {
        let cmd_res = process_cmd(&self.store_srv.store,cmd_req).await;
        cmd_res
    }
}
// 实现Clone trait
impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self { store_srv: self.store_srv.clone() }
    }
}

async fn process_cmd(store: &impl Storage,cmd_req: CmdRequest) -> CmdResponse {
    match cmd_req.req_data {
        Some(ReqData::Get(cmd_get)) => {
            cmd_get.excute(store)
        },
        Some(ReqData::Set(cmd_set)) => {
            cmd_set.excute(store)
        }
    _ => "Invalid Command".into(),    
    }
}