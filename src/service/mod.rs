mod cmd_service;

use crate::CmdResponse;
use crate::Storage;

// 定义命令执行公共行为
pub trait CmdService {
    // 命令执行方法  &impl Storage 与&dyn Storage的区别 ？ TODO
    fn excute(self,stor: &dyn Storage) -> CmdResponse;
}
