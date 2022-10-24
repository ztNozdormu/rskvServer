use std::{fs, error::Error};
use serde::{Serialize, Deserialize};



#[derive(Debug,Serialize,Deserialize)]
pub struct ServerConfig {
  pub listen_address : ListenAddress,
  pub rocks_db_path: RocksDbPath,
  pub connects: MaxConnects
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ListenAddress{
  pub  addr: String,
}

// rocksdbPath
#[derive(Debug,Serialize,Deserialize)]
pub struct RocksDbPath {
  pub rocks_db_path: String,
}

// 最大连接数配置
#[derive(Debug,Serialize,Deserialize)]
pub struct MaxConnects {
  pub max_connects: usize,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ClientConfig {
  pub connect_address: ConnectAddress,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ConnectAddress{
  pub addr: String,
}

impl ServerConfig {
  pub fn load(path: &str) -> Result<Self,Box<dyn Error>> {
        let  content = fs::read_to_string(path)?;
        let  server_config: Self = toml::from_str(&content)?;
        Ok(server_config)
  }
} 

impl ClientConfig {

  pub fn load(path: &str) -> Result<Self, Box<dyn Error>> { 

    let content = fs::read_to_string(path)?;
    let client_conf: Self = toml::from_str(&content)?;
    Ok(client_conf)
  }

}