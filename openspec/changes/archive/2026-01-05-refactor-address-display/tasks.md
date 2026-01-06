# Tasks: Refactor Address Display

1.  **准备阶段**
    - [x] 在 `Cargo.toml` 中添加 `hostname` 依赖。
    - [x] 验证：`make build` 成功。

2.  **实现主机名获取逻辑**
    - [x] 在 `src/main.rs` 中获取主机名并处理错误情况。
    - [x] 验证：在终端打印出正确的主机名。

3.  **更新界面显示 (Banner & TUI)**
    - [x] 修改 `src/main.rs` 的非交互模式输出逻辑，移除 `Local`，采用新格式。
    - [x] 修改 `src/dashboard.rs` 的渲染逻辑，展示主机名 + IP。
    - [x] 在 `DashboardState` 中增加 `hostname` 字段。
    - [x] 验证：运行 `target/release/air` 观察新格式是否生效。

4.  **适配 HTTPS 模式**
    - [x] 确保在开启 `--https` 时，地址前缀正确显示为 `https://`。
    - [x] 验证：运行 `target/release/air --https` 检查地址。
