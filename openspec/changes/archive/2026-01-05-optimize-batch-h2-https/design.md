# Design: Configurable HTTPS and HTTP/2 Optimization

## 1. 证书管理

我们将使用 `rcgen` 在内存中生成证书，不写入磁盘，确保“即插即用”：
- **Subject Alternative Name (SAN)**: 必须包含 `127.0.0.1` 和获取到的 `lan_ip`。
- **证书类型**: 自签名 (Self-signed)。

## 2. 依赖选择

- `axum-server`: 用于在同一个端口或指定端口方便地启动 Rustls 监听。
- `rcgen`: 生成符合现代浏览器要求的证书。
- `rustls`: 作为 TLS 后端。

## 3. 服务器启动逻辑重构

在 `src/server.rs` 中：
- `start` 函数将接收一个新参数 `enable_https: bool`。
- 如果 `enable_https` 为 true：
    1. 调用证书生成模块。
    2. 使用 `axum_server::bind_rustls` 绑定地址。
- 如果为 false：
    1. 使用现有的 `axum::serve`。

## 4. CLI 参数设计

```rust
#[arg(long, default_value_t = false)]
https: bool,
```

## 5. TUI 呈现

在 `src/dashboard.rs` 中：
- 地址前缀根据配置显示 `http://` 或 `https://`。
- 增加一个标签显示 `[H2 Multiplexing Enabled]`。
