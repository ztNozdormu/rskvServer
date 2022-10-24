#[warn(non_snake_case)]
use std::error::Error;
use anyhow::Result;
use kvserver::ServerConfig;
use kvserver::rocks_db_storage::RocksDbStorage;
use kvserver::server::Server;
use kvserver::service::{Service,StorageService};
use tokio::signal;
use tracing::{info, span};
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {
    // 测量监控逻辑
    let tracer = opentelemetry_jaeger::new_agent_pipeline().with_service_name("kv_server").install_simple()?;
    let tracing_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    tracing_subscriber::registry().with(EnvFilter::from_default_env()).with(tracing_layer).init();
    let root = span!(tracing::Level::INFO,"ap_start",work_units=2);
    let _enter = root.enter();

    let server_config = ServerConfig::load("src/conf/server.conf")?;
    let listen_address = server_config.listen_address.addr;
    let rocks_db_path = server_config.rocks_db_path.rocks_db_path;
    let max_connects = server_config.max_connects.max_connects;

    // 初始化Service及存储 默认使用RocksDbStorage
    let service: Service = StorageService::new(RocksDbStorage::new(rocks_db_path))
                .register_recv_event(|req|info!("[DEBUG] Receive req: {:?}",req))
                .register_exec_event(|res|info!("[DEBUG] Receive req: {:?}",res))
                .register_res_event(|res|info!("[DEBUG] Receive req: {:?}",res))
                .into();
    let server = Server::new(listen_address, service,max_connects);
    // 监听ctrl+c信号
    server.run(signal::ctrl_c()).await
}


