# Tasks: Implement TUI Dashboard

1.  **依赖配置**
    - [x] 在 `Cargo.toml` 中添加 `ratatui` 和 `crossterm` 依赖。
    - [x] 验证：`cargo build` 成功。

2.  **状态与日志捕获重构**
    - [x] 在 `handlers::Stats` 中增加 `logs: Mutex<VecDeque<String>>`。
    - [x] 修改 `logger::log_request` 以将日志推送到队列，不再直接 `println!`。
    - [x] 验证：单元测试或临时打印确认日志已进入队列。

3.  **实现 TUI 仪表盘组件**
    - [x] 创建 `src/dashboard.rs`。
    - [x] 实现 `Header` 渲染逻辑（固定信息）。
    - [x] 实现 `LogList` 渲染逻辑（滚动显示）。
    - [x] 验证：使用测试数据渲染预览。

4.  **主流程与信号集成**
    - [x] 重构 `main.rs` 以支持 TUI 终端初始化和清理。
    - [x] 实现异步 UI 循环，处理按键输入（'q' 退出，上下滚动）。
    - [x] 修改 `server::start` 以支持外部关闭信号。
    - [x] 验证：按下 'q' 键能正常退出 TUI 并返回控制台。

5.  **收尾与统计打印**
    - [x] 确保 TUI 退出后，在控制台打印最终的统计摘要。
    - [x] 验证：完整流程测试，确认统计数据准确无误。
