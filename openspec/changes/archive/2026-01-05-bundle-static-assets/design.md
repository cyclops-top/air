# Design: Bundling Static Assets

## 1. 资源嵌入

在 `src/handlers.rs` 中，定义一个静态常量来存储嵌入的文件内容：
```rust
const FAVICON_SVG: &[u8] = include_bytes!("../docs/favicon.svg");
```

## 2. Handler 修改

修改 `favicon` 函数：
- 不再使用 `ServeFile`（因为它设计用于服务磁盘文件）。
- 使用 Axum 的内置响应构建器直接返回字节流。
- 显式设置 `Content-Type: image/svg+xml`。

```rust
pub async fn favicon() -> impl IntoResponse {
    (
        [(axum::http::header::CONTENT_TYPE, "image/svg+xml")],
        FAVICON_SVG,
    )
}
```

## 3. 依赖考量
无需引入额外的 crate（如 `rust-embed`），因为目前只有一个小型的 SVG 文件，简单的 `include_bytes!` 足矣。
