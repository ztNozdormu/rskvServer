mod cmd_service;

use std::sync::Arc;

use tracing::info;

use crate::CmdRequest;
use crate::CmdResponse;
use crate::Storage;
use crate::cmd_request::ReqData;
use crate::pb::cmd;
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
    on_recv_event: Vec<fn(&CmdRequest)>,
    on_exec_event: Vec<fn(&CmdResponse)>,
    on_res_event: Vec<fn(&CmdResponse)>,
} 

impl<Store> StorageService<Store> {
    fn new(store: Store) -> Self {
        Self { 
            store,
            on_recv_event: Vec::new(),
            on_exec_event: Vec::new(), 
            on_res_event: Vec::new() 
        }
    }

    // ===通知事件注册函数==
    pub fn register_recv_event(mut self,f: fn(&CmdRequest)) -> Self {
        self.on_recv_event.push(f);
        self
    }

    pub fn register_exec_event(mut self,f: fn(&CmdResponse)) -> Self {
        self.on_exec_event.push(f);
        self
    }

    pub fn register_res_event(mut self,f: fn(&CmdResponse)) -> Self {
        self.on_res_event.push(f);
        self
    }

    // 通知事件执行函数
    pub async fn notify_recv_event(&self,cmd_req: &CmdRequest){
        self.on_recv_event.iter().for_each(|f|f(cmd_req));
    }
    pub async fn notify_exec_event(&self,cmd_res: &CmdResponse){
        self.on_exec_event.iter().for_each(|f|f(cmd_res));
    }
    pub async fn notify_res_event(&self,cmd_res: &CmdResponse){
        self.on_res_event.iter().for_each(|f|f(cmd_res));
    }
}

impl <Store: Storage> Service<Store> {
    
    pub fn new(store: Store) -> Self {
        Self { 
            store_srv: Arc::new(StorageService::new(store))}
    }

    pub async fn excute(&self,cmd_req: CmdRequest) -> CmdResponse {
        info!("Receive command request: {:?}", cmd_req);
        self.store_srv.notify_recv_event(&cmd_req).await;
        let cmd_res = process_cmd(&self.store_srv.store,cmd_req).await;
        info!("Execute command, response: {:?}", cmd_res);
        self.store_srv.notify_exec_event(&cmd_res).await;
        self.store_srv.notify_res_event(&cmd_res).await;
        info!("Response CmdResponse before");
        cmd_res
    }

}
// 实现Clone trait
impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self { store_srv: self.store_srv.clone() }
    }
}
// 实现从StorageService<Store>转换为Service<Store>
impl<Store: Storage> From<StorageService<Store>> for Service<Store> {
    fn from(store: StorageService<Store>) -> Self {
        Self { store_srv: Arc::new(store) }
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

