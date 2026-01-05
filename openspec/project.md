# Project Context

## Purpose
Air 是一个轻量级、安全的局域网文件共享 CLI 工具，旨在快速将本地目录映射为支持断点续传的 HTTP 文件服务器。

## Tech Stack
- **语言**: Rust (2021 edition)
- **Web 框架**: Axum
- **异步运行时**: Tokio
- **CLI 解析**: Clap
- **构建工具**: Make

## Project Conventions

### Code Style
- 遵循 Rust 标准风格。
- 使用 `make fmt` 进行格式化。
- 使用 `make lint` (clippy) 进行静态分析。

### Architecture Patterns
- **src/main.rs**: CLI 入口与启动横幅。
- **src/server.rs**: Axum 路由与服务器启动逻辑。
- **src/handlers.rs**: 请求处理逻辑（目录列表与文件服务）。
- **src/fs_utils.rs**: 文件系统工具与安全沙箱检查。
- **src/view.rs**: HTML 模板渲染。
- **src/logger.rs**: 自定义请求日志中间件。

### Testing Strategy
- 核心逻辑（如路径安全检查、面包屑生成）需包含单元测试。
- 使用 `make test` 运行所有测试。

## Important Constraints
- **安全沙箱**: 严格限制访问范围，禁止路径穿越（Path Traversal）。
- **隐藏文件**: 默认不展示以 `.` 开头的文件。

## External Dependencies
- `axum`, `tokio`, `clap`, `tower-http`, `chrono`, `local-ip-address`.