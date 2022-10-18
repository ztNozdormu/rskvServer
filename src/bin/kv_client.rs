use std::error::Error;
use anyhow::Result;
use bytes::BytesMut;
use clap::Parser;
use futures::{SinkExt, StreamExt};
use kvserver::{ClientConfig, CmdRequest, CmdResponse, args::ClientArgs};
use prost::Message;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::info;


#[tokio::main]
async fn main() ->Result<(),Box<dyn Error>> {
  
  tracing_subscriber::fmt::init();

  let client_config = ClientConfig::load("src/conf/client.conf")?;
  let addr = client_config.connect_address.addr;//"127.0.0.1:19999";


  let tcp_stream = TcpStream::connect(&addr).await?;

  // println!("客户端连接到服务器,服务器地址:[{}]",addr);
  // tcpStream.write(b"I am a goog coin").await?;

  // let mut buffer = vec![0u8;1024];
  // let length =  tcpStream.read(&mut buffer).await.expect("读取信息失败!");

  // println!("客户端读取到的内容[{}],内容长度:[{}]",String::from_utf8(buffer).unwrap(),length);
    // 解析命令行输入的内容
    let client_args = ClientArgs::parse();

    // 使用Frame的LengthDelimitedCodec进行编解码操
    let mut stream = Framed::new(tcp_stream,LengthDelimitedCodec::new());
    let mut buf = BytesMut::new();
    // 创建 get 命令
    // let cmd_get= CmdRequest::get("key");
    // 解析命令行输入的内容为客户端命令
    let cmd_get = process_args(client_args).await?;
    cmd_get.encode(&mut buf).unwrap();
    info!("send message success!");
    // 发送get命令消息
    stream.send(buf.freeze()).await.unwrap();
    // 接收服务器响应消息
    while let Some(Ok(buf)) = stream.next().await {
      let cmd_res = CmdResponse::decode(&buf[..]).unwrap();
      info!("Recived server response {:?}", cmd_res);
    }
   
  Ok(())
}

async fn process_args(args: ClientArgs) -> Result<CmdRequest,Box<dyn Error>> {

   match args {
    ClientArgs::Get { key } => {
      Ok(CmdRequest::get(key))
    },
    ClientArgs::Set { key, value } => {
      Ok(CmdRequest::set(key, value.into(),1000))
    },
    ClientArgs::Publish { topic, value } => {
      Ok(CmdRequest::publish(topic, value.into()))
    },
    ClientArgs::Subscribe { topic } => {
      Ok(CmdRequest::subscribe(topic))
    },
    ClientArgs::UnSubscribe { topic, id } => {
      Ok(CmdRequest::unsubscribe(topic, id.into()))
    }
   } 
   
}