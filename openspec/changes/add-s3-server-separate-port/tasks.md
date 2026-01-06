# Tasks: Implement S3 Server on Separate Port

1.  **准备阶段**
    - [ ] 在 `Cargo.toml` 中添加 `s3s` 和 `s3s-aws` 依赖。
    - [ ] 验证：`make build` 成功。

2.  **实现 S3 后端逻辑**
    - [ ] 创建 `src/s3.rs`。
    - [ ] 实现 `s3s::S3` trait，核心功能：`list_objects_v2`, `get_object`, `head_object`。
    - [ ] 确保 `get_object` 能复用现有的安全检查和缓存逻辑。
    - [ ] 验证：编译无误。

3.  **重构 Server 启动逻辑**
    - [ ] 修改 `src/server.rs` 以支持双端口监听。
    - [ ] 为 S3 端口实现自动随机选择和重试逻辑。
    - [ ] 启动独立的 `s3s` 监听任务。
    - [ ] 验证：启动后能看到两个不同的监听端口。

4.  **适配 CLI 与 TUI**
    - [ ] 修改 `src/main.rs` 的 `Cli` 结构体，添加 `--s3-port` 参数。
    - [ ] 修改 `src/dashboard.rs` 和 `DashboardState` 以展示双地址。
    - [ ] 验证：全屏模式下清晰显示两个 Endpoint 信息。

5.  **跨平台功能验证**
    - [ ] 使用 `aws s3 ls --endpoint-url http://localhost:<s3_port>` 验证功能。
    - [ ] 使用 `rclone` 尝试挂载或同步。
