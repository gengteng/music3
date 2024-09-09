# Music3 backend

该 Rust 项目使用工作空间（workspace）管理多个子项目。包括：

* [crates/common](crates/common): 库，前后端公用的公共模块，包括参数、模型结构定义，以及共用的工具函数等。
* [crates/server](crates/server): 库，后端的主要业务逻辑，不包括启动器。
* [crates/client](crates/client): 库，前端的主要业务逻辑，可通过 flutter_rust_bridge 给客户端使用。
* [crates/shuttle](crates/shuttle/): 可执行程序，用于发布到 shuttle.rs 的二进制运行入口。

# 开发

## 环境安装

1. 安装 Rust 工具链：https://www.rust-lang.org/tools/install
2. 安装 **cargo-shuttle** ，以便将后端服务发布到 shuttle.rs：https://docs.shuttle.rs/getting-started/installation

## 本地开发

1. 在 `backend/crates/shuttle` 下执行 `cargo shuttle project start --name <project-name> --idle-minutes 0`，自己给项目起一个名字。
2. 在 `backend/crates/shuttle` 下执行 `cargo shuttle run`，启动后端服务。

## 部署

1. 在 `backend/crates/shuttle` 下执行 `cargo shuttle deploy --name <project-name>`，发布后端服务到 shuttle.rs。