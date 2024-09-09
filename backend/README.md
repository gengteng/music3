# Music3 backend

该 Rust 项目使用工作空间（workspace）管理多个子项目。包括：

* [crates/common](crates/common): 库，前后端公用的公共模块，包括参数、模型结构定义，以及共用的工具函数等。
* [crates/server](crates/server): 库，后端的主要业务逻辑，不包括启动器。
* [crates/client](crates/client): 库，前端的主要业务逻辑，可通过 flutter_rust_bridge 给客户端使用。
* [crates/shuttle](crates/shuttle/): 可执行程序，用于发布到 shuttle.rs 的二进制运行入口。