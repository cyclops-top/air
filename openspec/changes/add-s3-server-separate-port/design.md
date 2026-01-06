# Design: S3 Server on Separate Port

## 1. 双端口启动逻辑

在 `src/server.rs` 中，`start` 函数将演进为管理两个服务：
1. **Web 服务**: 运行现有的 Axum 路由（HTML/JSON/Files）。
2. **S3 服务**: 运行基于 `s3s` 的对象存储服务。

```rust
pub async fn start(
    port: Option<u16>,
    s3_port: Option<u16>,
    // ...
) -> Result<(Arc<AppState>, u16, u16)>
```

## 2. S3 后端实现 (src/s3.rs)

我们将实现一个自定义的 S3 Backend，将 S3 请求映射到本地文件系统：
- **ListBuckets**: 返回虚拟桶列表。
- **ListObjectsV2**: 调用本地目录扫描逻辑。
- **GetObject**: 利用现有的流式读取、Range 和 ETag 逻辑。
- **HeadObject**: 返回文件元数据。

## 3. 自动端口选择

S3 端口也将支持随机选择逻辑，并确保不与主端口冲突：
- 范围同为 `10000-65535`。
- 循环尝试直到成功绑定。

## 4. TUI 集成

更新 `DashboardState` 以存储两个端口：
- `web_port`: 用于 UI。
- `s3_port`: 用于 API。

在渲染时：
```text
  ➜ Web UI: http://hostname.local:12345
  ➜ S3 API: http://hostname.local:54321
```
