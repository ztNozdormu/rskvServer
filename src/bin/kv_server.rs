#[warn(non_snake_case)]
use std::error::Error;
use anyhow::Result;
use bytes::Bytes;
use futures::StreamExt;
use futures::SinkExt;
use kvserver::{ServerConfig, CmdRequest, CmdResponse};
use prost::Message;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {
     
    tracing_subscriber::fmt::init();

    let server_config = ServerConfig::load("src/conf/server.conf")?;//"127.0.0.1:19999";
    let addr = server_config.listen_address.addr;
    let listener = TcpListener::bind(&addr).await?;
    println!("服务器端启动,启动服务地址:[{}]",addr);
    loop {
        let (tcp_stream,addr) = listener.accept().await.expect("读取信息失败!");
        
        tokio::spawn(async move {

            // let mut buffer = vec![0u8;1024];
            // loop {
            //     let length = tcpStream.read(&mut buffer).await.expect("读取信息失败!");
           
            //     if length == 0 {
            //         return
            //     }

            //     tcpStream
            //     .write_all(&buffer[0..length])
            //     .await
            //     .expect("向socket写入数据失败!");
            //     print!("服务端读取到数据长度:[{}]",length);
            // }

            // 使用Frame的LengthDelimitedCodec进行编解码操作
            let mut stream= Framed::new(tcp_stream,LengthDelimitedCodec::new());
            while let Some(Ok(mut buf)) = stream.next().await {
                  // 对客户端发来的protobuf请求命令进行拆包
                  let cmd_req = CmdRequest::decode(&buf[..]).unwrap();
                  info!("recived client command:{:?}",cmd_req);
                  
                  buf.clear();

                  // 对protobuf的请求响应进行封包，然后发送给客户端。
                  let cmd_res = CmdResponse::new(200,"success".to_string(),Bytes::default());
                  cmd_res.encode(&mut buf).unwrap();
                  stream.send(buf.freeze()).await.unwrap();
            }
            info!("Client{:?} disconnected",addr);
        });
    }

}

