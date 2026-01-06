# Tasks: Implement Auto Port Selection

1.  **准备阶段**
    - [x] 在 `Cargo.toml` 中添加 `rand` 依赖.
    - [x] 验证：`make build` 成功.

2.  **重构 Server 启动逻辑**
    - [x] 修改 `src/server.rs` 中的 `start` 函数，接收 `Option<u16>` 类型的端口.
    - [x] 实现端口重试循环逻辑.
    - [x] 更新 `start` 函数返回选中的端口.
    - [x] 验证：单元测试模拟端口占用，确保随机选择逻辑生效.

3.  **适配 CLI 与 TUI**
    - [x] 修改 `src/main.rs` 中的 `Cli` 结构体，移除 `port` 的默认值.
    - [x] 更新 `main.rs` 调用 `server::start` 的方式，获取并使用实际端口.
    - [x] 确保 `DashboardState` 显示的是实际选定的端口.
    - [x] 验证：运行 `target/release/air`（不加参数）能随机启动在 10000+ 端口.

4.  **最终集成验证**
    - [x] 同时启动两个 `air` 实例而不指定端口，验证它们能自动避开冲突并分别启动.
