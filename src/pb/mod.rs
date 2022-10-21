use bytes::Bytes;
use std::error::Error;
use crate::{cmd_request::ReqData, CmdRequest, Get, Publish, Set, Subscribe, Unsubscribe, CmdResponse};

pub mod cmd;
 
impl CmdRequest {
     // GET命令
     pub fn get(key: impl Into<String>) -> Self {
        Self {
            req_data: Some(ReqData::Get(Get { key: key.into() })),
        }
    }

    // SET命令
    pub fn set(key: impl Into<String>, value: Bytes, expire: u32) -> Self {
        Self {
            req_data: Some(ReqData::Set(Set {
                key: key.into(),
                value,
                expire,
            })),
        }
    }

    // PUBLISH命令
    pub fn publish(topic: impl Into<String>, value: Bytes) -> Self {
        Self {
            req_data: Some(ReqData::Publish(Publish {
                topic: topic.into(),
                value,
            })),
        }
    }

    // 订阅命令
    pub fn subscribe(topic: impl Into<String>) -> Self {
        Self {
            req_data: Some(ReqData::Subscribe(Subscribe {
                topic: topic.into(),
            })),
        }
    }

    // 解除订阅命令
    pub fn unsubscribe(topic: impl Into<String>, id: u32) -> Self {
        Self {
            req_data: Some(ReqData::Unsubscribe(Unsubscribe {
                topic: topic.into(),
                id,
            })),
        }
    }
}

impl CmdResponse {
    pub fn new(status: u32, message: String, value: Bytes) -> Self {
        Self { 
            status, 
            message, 
            value,
        }
    }
}

    // 实现从Bytes、&str、Box<dyn Error>转换为CmdResponse
    impl From<Bytes> for CmdResponse {
        fn from(bytes: Bytes) -> Self {
            Self {
                status: 200u32,
                message: "success".to_string(),
                value: bytes 
            }
        }
    }
    
    impl From<&str> for CmdResponse {
        fn from(str: &str) -> Self {
            Self {  
                status: 400u32,
                message: str.to_string(),
                ..Default::default()
            }
        }
    }
    impl From<Box<dyn Error>> for CmdResponse {
        fn from(err: Box<dyn Error>) -> Self {
            Self {
                status: 500u32,
                message: err.to_string(),
                ..Default::default()
            }
        }
    }    