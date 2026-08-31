# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

`air` 是一个用 Rust 编写的轻量级局域网文件共享 CLI：把一个目录变成 HTTP/HTTPS 文件服务器，带全屏 TUI 仪表盘（图形终端下显示二维码扫码访问）、Web 文件列表 UI、HTTP Range 断点续传、mDNS 服务发现（`air discover` 子命令）。当前版本 1.2.3。

## 常用命令

项目用 Makefile 管理日常任务：

```bash
make build           # cargo build --release，产物 target/release/air
make run ARGS="."    # cargo run -- <args>，等价于 ./air .
make test            # cargo test
make fmt             # cargo fmt
make lint            # cargo clippy
make build-all       # 交叉编译 linux/macos/windows，产物输出到 build/
```

运行单个测试（现有测试在 `src/fs_utils.rs` 和 `src/view.rs`）：

```bash
cargo test test_traversal_attempt   # 按名字过滤
```

手工运行验证：

```bash
cargo run -- /path/to/share --port 9000   # 共享目录
cargo run -- /path/to/share --https        # HTTPS（自动生成自签名证书，解锁 HTTP/2）
cargo run -- discover                      # 交互式 mDNS 服务发现
```

发布：推送 `v*` 标签触发 GitHub Actions（`.github/workflows/release.yml`）交叉编译 4 个平台产物并发布到 GitHub Release。

## 架构

v1.2.2 起采用分层架构（clean architecture），依赖方向 `domain ← application ← infrastructure`，`main.rs` 在顶层组装。新增功能时应遵循：契约写进 domain trait，实现放 infrastructure，编排逻辑放 application。

- **domain**（`src/domain/`）— 纯类型与契约，不依赖上层。
  - `models.rs`：跨层共享类型 — `FileEntry`、`DirectoryListing`、`DiscoveryMsg`、`LogEntry`/`LogAction`、`Stats`、`AppState`（整个服务用 `Arc` 共享的状态容器）。
  - `traits.rs`：三个异步契约 — `FileRepository`（列目录/文件流/摘要）、`UiRenderer`（渲染 HTML）、`DiscoveryProvider`（mDNS 注册/注销/发现）。
- **application**（`src/application/`）— 用例编排，只依赖 domain trait。`FileService`（列表 + 文件内容与 ETag/统计）、`DiscoveryManager`。
- **infrastructure**（`src/infrastructure/`）— 具体实现。
  - `filesystem/local.rs`：`LocalFileRepository`，列表过滤隐藏文件（`.` 开头）、目录优先排序。
  - `discovery/mdns.rs`：`MdnsDiscoveryProvider`，服务类型 `_air-share._tcp.local.`，注册属性 `id`/`scheme`，含局域网 IP 选择逻辑。
  - `server/mod.rs`：axum `Router` + `start_server`。端口策略：用户指定 → 否则 9567 → 9568 → 随机端口。HTTPS 用 `axum-server` + `cert.rs` 自签名证书。
  - `server/handlers.rs`：核心 fallback 处理器。
  - `api/mod.rs`：JSON API（`/api/list/`、`/api/list/{*path}`）。
  - `ui/html_renderer.rs`：`UiRenderer` 实现，实际 HTML 渲染在 `view.rs`。
- **顶层**：`main.rs`（clap CLI + TUI 循环 + 组装）、`view.rs`、`dashboard.rs`、`cert.rs`、`fs_utils.rs`。

## 需要跨文件理解的关键点

- **Web UI 内嵌于源码**：`src/view.rs` 的 `render_html` 用字符串拼接生成整个 HTML/CSS/JS 页面（含内联 SVG 图标），不是静态文件。`docs/design-*.html` 只是设计参考稿。改动 UI 就是改这个函数。
- **URL 前缀 `/air/`**：Web UI 全部路径带 `/air/` 前缀。`/` → 临时重定向 `/air/`；`handlers.rs` 里 `internal_path = &uri_path[4..]` 硬编码切掉前缀（按字符下标切，不是按 `/air` 匹配——若改前缀必须同步这里）。**API 路由不带 `/air` 前缀**。
- **路由分工**：显式路由只有 `/favicon.ico`、`/api/list/`、`/api/list/{*path}`，其余全部走 fallback 的 `handle_request`。目录请求按 `Accept` 头协商：`application/json` → JSON，否则 HTML。
- **多 State 组合**：axum `Router::with_state` 只能有一个 state，`server/mod.rs` 用 `CombinedState` + 手写 `FromRef` 让 server 处理器拿 `ServerState`、API 处理器拿 `ApiState`。新增处理器若需要不同状态就走这个模式。
- **请求日志机制**：`stats_middleware` 拦截所有请求，写入 `AppState.stats.logs`（上限 100 条）。handler 在 response 的 extensions 里插入 `LogAction` 标记来记录某条请求 —— 给新端点加日志需要 `res.extensions_mut().insert(LogAction::Xxx)`。
- **ETag 策略**（`local.rs::get_file_digest`）：>10MB 文件用 `mtime+size` 弱哈希（Base64）避免读全文件阻塞；小文件 SHA-256 + `DashMap` 缓存。
- **Range/断点续传**：解析在 `handlers.rs::parse_range`，流式截断在 `get_file_stream`（`file.take()`）。
- **mDNS 生命周期**：`start_server` 拿到真实端口后才注册服务，退出时 `unregister_service`；`discover` 模式靠 tx channel drop 结束后台线程。
- **安全关键**：`fs_utils.rs::sanitize_path` 做 `canonicalize` + 前缀检查防路径穿越/符号链接逃逸，所有对外路径必须经它处理。
- **端口默认值以代码为准（9567）**；`docs/technical-specification.md` 里的 8000 是旧文档。
- 源码注释、README、文档均为中文；commit message 混用中英文并习惯带版本号（如 `feat: xxx v1.2.3`），版本号在 `Cargo.toml`。
- 遗留兼容代码：`local.rs` 尾部的 `MmapCache` 是空壳；`main.rs` 里 `AppState.port = 0` 是占位值，真实端口在 `start_server` 返回后才回填。

## 测试

现有测试集中在 `src/fs_utils.rs`（`sanitize_path` 的路径穿越/符号链接逃逸、SHA-256 向量）和 `src/view.rs`（`format_duration`），依赖 `tempfile`。HTTP handler 层目前没有集成测试。
