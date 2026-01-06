# Tasks: Implement Air in Rust

1.  **项目初始化**
    - [x] 使用 `cargo new air` 创建项目结构。
    - [x] 创建 `Makefile` 并实现 `build`, `test`, `run` 目标。
    - [x] 配置 `Cargo.toml` 添加依赖 (`axum`, `tokio`, `clap`, `anyhow` 等)。
    - [x] 验证：运行 `make build` 成功。

2.  **CLI 参数解析与启动横幅**
    - [x] 实现 `clap` 结构体，支持 `[path]` 和 `--port`。
    - [x] 集成 `local-ip-address` 获取局域网 IP。
    - [x] 实现启动 Banner 打印逻辑。
    - [x] 验证：运行 `./air --port 9090 /tmp` 显示正确端口和 IP。

3.  **核心 HTTP 服务框架搭建**
    - [x] 设置 `axum` Router。
    - [x] 实现自定义日志中间件（Middleware），格式：`[Time] [IP] [Method] [Path] [Status] [Duration]`。
    - [x] 验证：发起请求，控制台输出符合规范的日志。

4.  **安全模块与路径解析**
    - [x] 实现 `sanitize_path` 函数：解析绝对路径，处理符号链接，检查前缀。
    - [x] 编写单元测试覆盖：正常路径、`../` 穿越尝试、软链接指向外部等场景。
    - [x] 验证：`cargo test` 通过所有安全用例。

5.  **目录列表功能 (Directory Listing)**
    - [x] 实现目录读取逻辑，获取文件元数据（大小、类型）。
    - [x] 实现 HTML 渲染器（简约 UI，图标，面包屑）。
    - [x] 实现 JSON 响应（适配 OpenAPI Schema）。
    - [x] 验证：访问文件夹路径，浏览器显示 UI，`curl -H "Accept: application/json"` 返回 JSON。

6.  **文件下载与断点续传**
    - [x] 集成 `tower-http` 或手动处理文件流。
    - [x] 确保支持 `Range` 头（206 Partial Content）。
    - [x] 验证：使用 `curl -r 0-100` 下载文件片段。

7.  **集成测试与交付**
    - [x] 手动测试全流程：启动 -> 浏览目录 -> 下载文件 -> 尝试越权访问。
    - [x] 最终代码审查与清理。
