# Design: Automatic Port Selection and Retry

## 1. 端口选择策略

- **范围**: `10000` 到 `65535`。这个范围避开了大多数系统保留端口和常用的 Web 开发端口（如 80, 443, 3000, 8080）。
- **随机性**: 使用 `rand::thread_rng().gen_range(10000..=65535)`。

## 2. 绑定逻辑重构

在 `src/server.rs` 中，`start` 函数逻辑调整：

```rust
async fn try_bind(addr: SocketAddr) -> Result<TcpListener> {
    TcpListener::bind(addr).await
}

// 在 start 函数内部
let used_port = if let Some(p) = port {
    p
} else {
    loop {
        let p = rand::thread_rng().gen_range(10000..=65535);
        let addr = SocketAddr::from(([0, 0, 0, 0], p));
        if TcpListener::bind(addr).await.is_ok() {
            break p;
        }
    }
};
```

## 3. HTTPS 处理

对于 HTTPS 模式（使用 `axum-server`），由于它也需要绑定 TCP，我们将采用先探测端口可用性，然后再交给 `axum-server` 绑定的方式，或者直接利用其错误处理。为了统一，建议先在 `start` 函数开始处确定 `used_port`。

## 4. 返回值变更

`server::start` 必须返回最终选定的端口号，以便 `main.rs` 和 `DashboardState` 能够显示正确的地址。

```rust
pub async fn start(...) -> Result<(Arc<AppState>, u16)>
```
