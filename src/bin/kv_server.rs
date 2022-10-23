#[warn(non_snake_case)]
use std::error::Error;
use anyhow::Result;
use kvserver::ServerConfig;
use kvserver::rocks_db_storage::RocksDbStorage;
use kvserver::server::Server;
use kvserver::service::{Service,StorageService};
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {

    tracing_subscriber::fmt::init();

    let server_config = ServerConfig::load("src/conf/server.conf")?;//"127.0.0.1:19999";
    let listen_address = server_config.listen_address.addr;
    let rocks_db_path = server_config.rocks_db_path.rocks_db_path;

    // 初始化Service及存储 默认使用RocksDbStorage
    let service: Service = StorageService::new(RocksDbStorage::new(rocks_db_path))
                .register_recv_event(|req|info!("[DEBUG] Receive req: {:?}",req))
                .register_exec_event(|res|info!("[DEBUG] Receive req: {:?}",res))
                .register_res_event(|res|info!("[DEBUG] Receive req: {:?}",res))
                .into();
    let server = Server::new(listen_address, service);

    server.run(signal::ctrl_c()).await

}


