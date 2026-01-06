# Tasks: Implement HTTPS and H2 Optimization

1.  **添加依赖**
    - [x] 在 `Cargo.toml` 中添加 `axum-server`, `rcgen`, `rustls-pemfile`。
    - [x] 验证：`make build` 成功。

2.  **实现证书生成逻辑**
    - [x] 在 `src` 下新建 `cert.rs`。
    - [x] 实现生成包含 LAN IP 的自签名证书函数。
    - [x] 验证：测试生成的证书包含正确的 SAN。

3.  **重构服务器启动逻辑**
    - [x] 修改 `src/server.rs` 以支持 TLS 配置。
    - [x] 修改 `src/main.rs` 解析 `--https` 参数。
    - [x] 验证：运行 `target/release/air --https` 能够通过 `https://` 访问。

4.  **TUI 界面适配**
    - [x] 更新 `dashboard.rs` 显示当前使用的协议。
    - [x] 确保地址链接正确。
    - [x] 验证：TUI 中正确显示 `https://` 路径。

5.  **批量请求性能测试 (可选验证)**
    - [x] 对比启用 H2 前后的多资源加载耗时。
