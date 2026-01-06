# Design: TUI Dashboard Architecture

## 1. 技术栈
- **渲染引擎**: `ratatui`
- **终端后端**: `crossterm`
- **并发控制**: `tokio` + `std::sync::Arc`

## 2. 数据流设计

### 2.1 日志捕获
`src/logger.rs` 中的 `log_request` 中间件将捕获请求详情，并将其格式化为字符串。
这些字符串将被推送到 `AppState.stats.logs`（一个 `Arc<Mutex<VecDeque<String>>>`）。

### 2.2 UI 渲染循环
`main.rs` 将负责启动 TUI 终端：
1.  进入原始模式 (Raw mode) 并切换到替代屏幕 (Alternate screen)。
2.  循环渲染：
    -   **Header**: 显示路径、IP、端口、状态。
    -   **Logs Window**: 显示 `VecDeque` 中的最近日志，支持滚动偏量 (scroll offset)。
3.  处理输入：
    -   `q`: 发送退出信号。
    -   `Up/Down`: 调整日志滚动位置。

### 2.3 异步协调
-   **Server Task**: 在 `tokio::spawn` 中运行 Axum 服务器。
-   **UI Task**: 在主线程（或另一个 task）中运行 TUI 循环。
-   **Shutdown**: 使用 `tokio::sync::oneshot` 或 `watch` 频道在按下 'q' 时通知服务器关闭。

## 3. UI 布局
```text
+-------------------------------------------------------------+
| Air File Server - Serving at /Users/justin/Downloads        |
| Local: http://localhost:8000 | Network: http://1.2.3.4:8000 |
| [Stats] Files: 12 | Volume: 4.5 MB | Sandbox: ENABLED       |
+-------------------------------------------------------------+
| [15:30:01] 192.168.1.5 GET /index.html 200 (OK) - 2ms       |
| [15:30:05] 192.168.1.5 GET /style.css 200 (OK) - 1ms        |
| ... (more logs)                                             |
+-------------------------------------------------------------+
| Press 'q' to quit, Up/Down to scroll                        |
+-------------------------------------------------------------+
```

## 4. 退出逻辑
1.  用户按 'q'。
2.  TUI 循环终止。
3.  清理终端状态（退出原始模式，切回主屏幕）。
4.  等待服务器优雅关闭。
5.  从 `stats` 中读取最终数据并 `println!` 到控制台。
