# Tasks: Implement Dashboard Uptime Timer

1.  **更新 Stats 结构体**
    - [x] 在 `src/handlers.rs` 中为 `Stats` 增加 `start_time: std::time::Instant` 字段。
    - [x] 确保 `Default` 实现正确初始化该时间。
    - [x] 验证：代码编译通过。

2.  **实现时长格式化工具**
    - [x] 在 `src/view.rs` 中增加 `format_duration` 函数。
    - [x] 验证：单元测试验证不同时间长度（秒、分、时）的格式化结果。

3.  **集成到 TUI 渲染**
    - [x] 修改 `src/dashboard.rs`，计算 `elapsed()` 并调用格式化工具。
    - [x] 在 Header 区域展示 `Uptime` 信息。
    - [x] 验证：启动 `air` 进入 TUI，观察时间每秒更新。

4.  **更新最终统计打印**
    - [x] 修改 `src/main.rs`，在 Summary 输出中增加 `Total uptime` 项。
    - [x] 验证：按 'q' 退出后，能看到正确的总运行时长。
