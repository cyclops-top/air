# Design: S3 Server Implementation on Axum

## 1. 路由设计 (Shared Port Strategy)

在 `src/server.rs` 中，我们将配置 Axum 路由进行分流：
- `GET /` (及大部分路径): 映射到现有的 Web UI / 文件服务。
- `ANY /_s3/*path`: 映射到 `s3s` 的服务入口。

```rust
let s3_service = S3Service::new(custom_s3_backend);
let app = Router::new()
    .route("/_s3/*path", any(s3_handler)) // S3 专用路径
    .fallback(get(handle_request));       // 现有 Web 逻辑
```

## 2. 后端适配 (S3 Backend)

实现 `s3s::S3` trait：
- **Bucket 模拟**: 提供一个名为 `default` 的虚拟桶，代表共享根目录。
- **路径转换**: `/_s3/default/folder/file.txt` 内部映射到 `ROOT_PATH/folder/file.txt`。
- **ETag**: 注入之前实现的 SHA-256 摘要作为 S3 的 `ETag` 响应头。

## 3. 冲突规避 (Collision Avoidance)

采用 `/_s3/` 前缀可以确保：
1. 用户即使分享了一个叫 `S3` 的文件，其 URL 将是 `http://ip:port/S3`，而 API 地址是 `http://ip:port/_s3`，互不干扰。
2. 保持根路径 `/` 依然服务于人类可读的 Web 界面。

## 4. TUI 适配

在全屏仪表盘中，增加一个指示器：
`[API] S3 Enabled: http://<ip>:<port>/_s3`
这样用户可以方便地复制 API 地址。
