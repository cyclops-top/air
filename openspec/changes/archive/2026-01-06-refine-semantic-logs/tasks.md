# Tasks: Refine Semantic Logs

1.  **重构数据模型**
    - [x] 将 `LogAction` 枚举移动到 `src/handlers.rs`。
    - [x] 在 `src/handlers.rs` 中定义 `LogEntry` 结构体。
    - [x] 将 `Stats.logs` 的类型从 `VecDeque<String>` 修改为 `VecDeque<LogEntry>`。
    - [x] 验证：编译通过。

2.  **更新日志收集逻辑**
    - [x] 修改 `src/logger.rs` 中的 `log_request`：
        - 检查响应扩展，如果是 `Favicon` 则跳过记录。
        - 构造 `LogEntry` 并推送到队列。
    - [x] 验证：代码逻辑正确，不记录 Favicon。

3.  **重写 TUI 渲染逻辑**
    - [x] 修改 `src/dashboard.rs` 中的 `render` 函数：
        - 遍历 `LogEntry` 队列。
        - 使用 ` ratatui::text::Line` 和 `Span` 组合不同颜色的列。
        - 实现固定宽度对齐逻辑。
        - 实现路径截断逻辑。
    - [x] 验证：运行 `make run` 观察日志区域的对齐和颜色。

4.  **错误状态处理**
    - [x] 在渲染时，根据 `is_success` 状态，为路径添加 `✖` 前缀并设为红色。
    - [x] 验证：访问不存在的路径，观察 TUI 日志显示。
