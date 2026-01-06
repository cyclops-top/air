# Design: ETag and Conditional Requests

## 1. ETag 生成策略

我们将直接使用 `implement-instance-digests` 中生成的 `SHA-256` Base64 字符串作为 ETag。
为了符合 HTTP 规范（RFC 7232），强 ETag 必须包含在双引号内。

示例：
- Hash: `uU0nuZNN...`
- ETag: `"uU0nuZNN..."`

## 2. 校验逻辑 (src/handlers.rs)

在 `handle_request` 中，获取到 `hash` 后：

```rust
let etag = format!("\"{}\"", hash);

if let Some(if_none_match) = headers.get(header::IF_NONE_MATCH) {
    if if_none_match == etag.as_str() {
        return StatusCode::NOT_MODIFIED.into_response();
    }
}
```

## 3. 响应头注入

所有文件下载响应（200 和 206）都将包含：
- `ETag: "..."`
- `Cache-Control: no-cache` (建议：强制客户端每次都向服务器验证，发挥 ETag 优势而不导致过度缓存)

## 4. 目录列表处理

虽然目录列表是动态生成的，但为了性能，我们也可以为 JSON/HTML 响应生成简单的 ETag（例如基于该目录下所有文件的修改时间之和），本提案初期优先支持文件 ETag。

```
