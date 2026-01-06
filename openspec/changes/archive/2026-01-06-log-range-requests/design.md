# Design: Logging Range Information

## 1. 数据模型变更

在 `src/handlers.rs` 的 `LogEntry` 中增加 `range` 字段：
```rust
pub struct LogEntry {
    pub time: String,
    pub ip: String,
    pub action: LogAction,
    pub duration: std::time::Duration,
    pub path: String,
    pub is_success: bool,
    pub range: Option<String>, // 新增
}
```

## 2. 头部提取逻辑

在 `src/logger.rs` 的 `log_request` 中：
```rust
let range_header = req.headers()
    .get(axum::http::header::RANGE)
    .and_then(|v| v.to_str().ok())
    .map(|s| s.replace("bytes=", "")); // 简化显示，去掉 bytes= 前缀
```

## 3. TUI 渲染适配

在 `src/dashboard.rs` 中展示日志行时：
- 如果 `entry.range` 有值：
    - 在 `path` 标签后增加展示内容。
    - 格式示例：`\xxxx\xxx.bin [0-500k]`。
- 考虑到宽度限制，如果 `range` 字符串过长，可以进行截断或使用更紧凑的表示。

## 4. 交互示例
```text
[10:00:01] 192.168.1.5 DOWNLOAD 500ms /movie.mp4 [0-1M]
[10:00:02] 192.168.1.5 DOWNLOAD 1s    /movie.mp4 [1-2M]
```
这样用户一眼就能看出是在进行连续的分段下载。
