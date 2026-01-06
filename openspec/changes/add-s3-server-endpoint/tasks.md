# Tasks: Implement S3 Server Endpoint

1.  **准备阶段**
    - [ ] 在 `Cargo.toml` 中添加 `s3s` 和 `s3s-aws` 依赖。
    - [ ] 验证：`make build` 成功。

2.  **实现 S3 后端逻辑**
    - [ ] 创建 `src/s3.rs`。
    - [ ] 实现 `s3s::S3` trait，核心功能：`list_objects_v2`, `get_object`, `head_object`。
    - [ ] 确保 `get_object` 能复用现有的安全检查逻辑。
    - [ ] 验证：编写针对 `s3.rs` 的测试用例。

3.  **集成到 Axum 路由**
    - [ ] 修改 `src/server.rs` 以集成 `s3s` 的 service。
    - [ ] 实现 `/_s3/` 前缀路由转发。
    - [ ] 验证：使用 `aws s3 ls --endpoint-url http://localhost:port/_s3` 验证是否能列出文件。

4.  **TUI 界面适配**
    - [ ] 在 `src/dashboard.rs` 中显示 S3 Endpoint 地址。
    - [ ] 验证：全屏模式下清晰显示 API 信息。

5.  **跨平台验证**
    - [ ] 使用 `rclone` 或 `Cyberduck` 连接 `Air` 模拟的 S3 服务进行文件浏览。
