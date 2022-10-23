##  KVserver
### 项目说明

  rust练习,tokio做底层异步网络通讯、使用toml文件做配置、protobuf做传输协议、内存/RockDB做数据存储、事件通知、优雅关机、并发连接限制及测量监控等
  TIKV分布式KV存储设计介绍:https://zhuanlan.zhihu.com/p/31142786 

### github开发项目初始化流程
 
 * 项目初始化参考文档:https://blog.csdn.net/qq_41990294/article/details/124631575
 * 操作流程
  ```   git init
        git add .//提交本地文件到暂存区
        //将暂存区内容添加到本地仓库中
        git commit -m"注释"
        // 关联远程仓库
        git remote add origin https://ghp安全码@github.com/ztNozdormu/rskvServer.git
        // 合并远程仓库readme.md文件
        git pull --rebase origin main
        // 切换并创建分支
        git checkout -b main
        // 推送代码到指定分支
        git push -u origin main


### 1.rust使用protoc

> + 系统ubuntu,建议git拉取最新版本进行安装.

* protoc安装操作

  ```   sudo apt-get install autoconf automake libtool curl make g++ unzip
        git clone -b v3.6.1 https://github.com/protocolbuffers/protobuf.git
        cd protobuf
        git submodule update --init --recursive
        ./autogen.sh
        ./configure
        make
        make check
        sudo make install
        sudo ldconfig # refresh shared library cache.
        protoc --version #查看版本
  

* 生成操作

  `项目下执行cargo build`

### kvServer 运行命令

1. 服务端运行命令 `RUST_LOG=info cargo run --bin kv_server`
2. 客户端运行命令 
`RUST_LOG=info cargo run --bin kv_client set --key name --value 999999`
`RUST_LOG=info cargo run --bin kv_client set --key age --value 18`
`RUST_LOG=info cargo run --bin kv_client get --key age`

### 其他项 


typro操作:
https://blog.csdn.net/supersuperboybai/article/details/105589309?spm=1001.2101.3001.6650.1&utm_medium=distribute.pc_relevant.none-task-blog-2%7Edefault%7ECTRLIST%7ERate-1-105589309-blog-123541079.t0_edu_mix&depth_1-utm_source=distribute.pc_relevant.none-task-blog-2%7Edefault%7ECTRLIST%7ERate-1-105589309-blog-123541079.t0_edu_mix&utm_relevant_index=1

### 练习进度
* 2022-10-14 Server-3 目标 在服务器端使用内存来存储客户端发送过来的数据。在实现数据存储之前，我们先在客户端使用Clap库来解析命令行参数，并封装成命令发送给服务器。
  1. 添加 Clap 库依赖,clap = { version = "4.0.15", , features = ["derive"]} 并指定features特性
  2. 定义ClientArgs 命令参数数据结构
  2. 定义存储模块和trait
  3. 实现内存存储功能 使用第三方库 dashmap 
  4. 测试 get set 命令
* 2022-10-20 server-4 封装命令执行service层 定义rocksDB存储功能
  1. 定义rocksDB模块 实现storage trait
  2. 创建service文件夹创建mod.rs, 定义CmdService trait
  3. 创建cmd_service.rs文件,其中为get set命令实现CmdService trait
  4. 实现从Bytes、&str、Box<dyn Error>转换为CmdResponse
  5. mod.rs中实现service层代码
  6. server.conf 中新增RocksDB配置(存储目录位置/tmp/kvserver),config.rs中新增rocksDB逻辑
  7. 在kv_server.rs中使用service执行命令，替换process_cmd函数
  8. 测试
* 2022-10-23
  1. 事件通知机制
     *  事件注册函数
     *  事件执行函数
  2. 优雅关机  
     *  当服务器端主线程收到Ctrl+c信号时，通过tokio的boardcast channel通知给所有的活跃连接；
     *  各子线程处理完业务逻辑或资源清理后通过tokio的mpsc channel通知给主线程；
     *  主线程停止运行，服务器关闭。






