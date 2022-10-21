use crate::CmdResponse;
use crate::Storage;

// 定义命令执行公共行为
pub trait CmdService {
    // 命令执行方法
    fn excute(self,stor: &Storage) -> CmdResponse;
}

// TODO 实现从Bytes、&str、Box<dyn Error>转换为CmdResponse