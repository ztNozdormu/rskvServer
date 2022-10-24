use std::{error::Error, sync::Arc};
use prost::Message;
use tokio::{net::TcpListener, sync::Semaphore};
use futures::{SinkExt, StreamExt, Future};
use tokio::{sync::{mpsc, broadcast}};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::{info, error, instrument};
use crate::{service::Service, CmdRequest};


pub struct Server{
    listen_address: String, // server 监听地址
    service: Service, // 业务逻辑Service
    max_connects: Arc<Semaphore> // 最大连接数配置
}

impl Server {

    pub fn new(listen_address: String, service: Service,max_connects: usize) -> Self {
        Self { listen_address, service,max_connects: Arc::new(Semaphore::new(max_connects)) }
    }

    // 与客户端建立链接
    #[instrument(name="server_execute",skip_all)]
    async fn execute(&self,notify_shutdown: &broadcast::Sender<()>,shutdown_complete_tx: &mpsc::Sender<()>) -> Result<(),Box<dyn Error>> {
        // 监听服务地址端口
        let listener = TcpListener::bind(&self.listen_address).await?;
        println!("服务器端启动,服务监听地址:[{}]",self.listen_address);
        loop {
            // 获取最大连接数
            let permit = self.max_connects.clone().acquire_owned().await.unwrap();
            // 监听客户端请求
            let (stream,addr) = listener.accept().await?;
            info!("客户端: {:?} 链接地址",addr);
            // 定义业务服务
            let serv = self.service.clone();
            // 定义通知线程(关闭通知/完成清理通知)
            let mut shutdown = notify_shutdown.subscribe();
            let shutdown_complete = shutdown_complete_tx.clone();
            // 创建线程执行业务
            tokio::spawn(async move {
                // 使用Frame的LengthDelimitedCodec进行编解码操作
                let mut stream= Framed::new(stream,LengthDelimitedCodec::new());
                drop(permit);
                // 循环读取客户端消息并处理
                loop {
                    let mut buf = tokio::select! {
                        Some(Ok(buf)) = stream.next() => {
                            buf
                        },
                        // 接收boardcast的关闭信息
                        _ = shutdown.recv() => {
                            // 清理工作
                            info!("服务进程关闭前进程资源清理 ......");
                            // 通知主线程处理完成
                            let _ = shutdown_complete.send(());
                            info!("服务进程关闭完成 ......");
                            return;
                        } 
                    };

                    // 对客户端发来的protobuf请求命令进行拆包
                    let cmd_req = CmdRequest::decode(&buf[..]).unwrap();
                    info!("接收到的客户端指令参数:{:?}",cmd_req);
                    let cmd_res = serv.excute(cmd_req).await;
                    buf.clear();
                    // 对protobuf的请求响应进行封包，然后发送给客户端。
                    cmd_res.encode(&mut buf).unwrap();
                    stream.send(buf.freeze()).await.unwrap();
                }   
            });
        }
    }
    // 监听SIGINT信号和监听客户端连接
    #[instrument(name="server_run",skip_all)]
    pub async fn run(&self, shutdown: impl Future) -> Result<(), Box<dyn Error>> {
        // 广播channel，用于给各子线程发送关闭信息
            let (notify_shutdown, _) = broadcast::channel(1);
            // mpsc channel，用于通知主线程，各子线程执行完成。
            let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel::<()>(1);
        
            tokio::select! {
            res = self.execute(&notify_shutdown, &shutdown_complete_tx) => {
                if let Err(err) = res {
                    error!(cause = %err, "接收信息失败!!!");
                }
            },
            // 接收Ctrl+c SIGINT
            _ = shutdown => {
                info!("KV Server 服务关闭!!!");
            }
        }

        drop(notify_shutdown);
        drop(shutdown_complete_tx);
        let _ = shutdown_complete_rx.recv().await;
        Ok(())
    }    
}

