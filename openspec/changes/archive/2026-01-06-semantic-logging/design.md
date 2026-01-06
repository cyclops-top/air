# Design: Semantic Logging Implementation

## 1. 语义化映射 (LogAction)

定义在 `src/logger.rs` 或 `src/handlers.rs`:
```rust
pub enum LogAction {
    OpenDir,
    DownloadFile,
    Favicon,
}
```

## 2. Handler 注入

在 `src/handlers.rs` 的 `handle_request` 中：
```rust
if metadata.is_dir() {
    let mut res = list_directory(...).await;
    res.extensions_mut().insert(LogAction::OpenDir);
    return res;
} else {
    // ... stats logic
    let mut res = ServeFile::new(abs_path).oneshot(req).await.into_response();
    res.extensions_mut().insert(LogAction::DownloadFile);
    return res;
}
```

## 3. Middleware 提取

在 `src/logger.rs` 的 `log_request` 中：
```rust
let response = next.run(req).await;
let action = response.extensions().get::<LogAction>();

let action_str = match action {
    Some(LogAction::OpenDir) => "OPEN DIR",
    Some(LogAction::DownloadFile) => "DOWNLOAD",
    Some(LogAction::Favicon) => "FAVICON",
    None => method.as_str(), // Fallback
};
```

## 4. 文本对齐与美化

为了保持 TUI 日志的整齐，语义化标签应尽量保持一致的宽度或使用简洁的英文/中文组合。
例如：
- `[OPEN DIR] /Movies`
- `[DOWNLOAD] /test.mp4`
- `[FAVICON] /favicon.ico`
