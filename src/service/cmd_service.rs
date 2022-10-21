use super::CmdService;
use crate::{Storage,CmdResponse,Get,Set};

// Get命令执行的实现
impl CmdService for Get {
    fn excute(self,stor: &dyn Storage) -> CmdResponse {
        match stor.get(&self.key) {
            Ok(Some(value)) => value.into(),
            Ok(None) => "None value".into(),
            Err(e) => e.into(),
        }
    }
}
// Set命令执行的实现
impl CmdService for Set {
    fn excute(self,stor: &dyn Storage) -> CmdResponse {
        match stor.set(&self.key,self.value) {
            Ok(Some(value)) => value.into(),
            Ok(None) => "set failed".into(),
            Err(e) => e.into(),
        }
    }
}