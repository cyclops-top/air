# Design: Refined Log Filtering and Decoding

## 1. 过滤逻辑

在 `src/logger.rs` 的 `log_request` 中间件中：
- 在 `next.run(req).await` 之前或之后，检查 `req.method()`。
- 如果 `method != http::Method::GET`，则直接返回响应，跳过构造 `LogEntry` 的逻辑。

## 2. 解码逻辑

使用现有的 `percent-encoding` 依赖：
```rust
use percent_encoding::percent_decode_str;

let decoded_path = percent_decode_str(&raw_path)
    .decode_utf8_lossy()
    .to_string();
```

## 3. 语义化动作集成

确保 `LogAction` 的提取仅在满足 `GET` 方法的前提下进行，以保持逻辑一致。
