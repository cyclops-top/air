# Tasks: Implement Range Request Logging

1.  **更新数据结构**
    - [x] 修改 `src/handlers.rs` 中的 `LogEntry` 结构体，添加 `range: Option<String>` 字段。
    - [x] 验证：编译通过。

2.  **提取 Range 信息**
    - [x] 修改 `src/logger.rs` 中的 `log_request` 中间件。
    - [x] 从 `req.headers()` 中读取 `axum::http::header::RANGE` 并简化存储。
    - [x] 验证：通过 `curl -r 0-100` 发起请求，确认 `LogEntry` 成功捕获 Range。

3.  **集成到 TUI 仪表盘**
    - [x] 修改 `src/dashboard.rs` 的渲染逻辑。
    - [x] 当 `range` 存在时，在 `Path` 动作后附加显示范围信息。
    - [x] 验证：启动 `air` 并播放视频或使用分段下载工具，观察日志对齐和内容。

4.  **优化显示格式**
    - [x] 验证：UI 视觉效果符合预期。
