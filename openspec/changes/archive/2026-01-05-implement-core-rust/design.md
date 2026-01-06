# Design: Air Rust 实现架构

## 1. 技术栈选择

*   **编程语言**: Rust (2021 edition)
*   **Web 框架**: `axum`
    *   原因：生态成熟，基于 Tokio，性能优异，易于处理 HTTP 协议细节（如 Range 头）。
*   **异步运行时**: `tokio`
*   **CLI 参数解析**: `clap` (derive 模式)
*   **IP 获取**: `local-ip-address`
*   **日志/追踪**: `tracing` + `tracing-subscriber` (或者自定义 middleware 以严格匹配格式要求)
*   **构建工具**: `make` (封装 `cargo`)

## 2. 架构设计

### 2.1 模块划分
```text
src/
├── main.rs          # 入口，CLI 解析，启动 Server
├── server.rs        # Axum Router 配置，Handler 定义
├── handlers.rs      # 具体业务逻辑 (文件服务, 目录渲染)
├── fs_utils.rs      # 文件系统操作，安全检查 (Path Sanitization)
├── view.rs          # HTML 模板/渲染逻辑
└── logger.rs        # 自定义日志中间件
```

### 2.2 核心流程

1.  **启动阶段**:
    *   `clap` 解析参数 (`port`, `path`)。
    *   `fs::canonicalize` 获取 `ROOT_PATH` 绝对路径。
    *   获取本机 LAN IP。
    *   打印 Startup Banner。
    *   启动 `axum` 服务监听端口。

2.  **请求处理 (Request Handling)**:
    *   **Middleware**: 记录请求开始时间。
    *   **Handler**: 提取 URL 路径。
    *   **Security Check**: 调用 `fs_utils::sanitize_path(root, request_path)`。
        *   若路径越界 -> 403。
    *   **Type Check**: `fs::metadata` 检查是文件还是目录。
    *   **Response**:
        *   **目录**: 生成 JSON 或 HTML (根据 Accept 头)。
        *   **文件**: 使用 `tower_http::services::ServeFile` 或手动实现流式传输以支持 `Range`。
    *   **Middleware**: 请求结束，计算耗时，按格式打印日志。

### 2.3 安全设计 (Path Traversal Prevention)
为满足规范中的 "Security Check: SANDBOX ENABLED"：
1.  启动时解析 `root_dir` 的 `canonical` 路径。
2.  对每个请求路径，与 `root_dir` 拼接后再次调用 `canonicalize` (解析 symlink)。
3.  检查结果是否以 `root_dir` 为前缀。
4.  如果是，允许访问；否则拒绝。

## 3. Makefile 接口
```makefile
.PHONY: build run test clean fmt lint

build:
	cargo build --release

run:
	cargo run -- $(ARGS)

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy
```
