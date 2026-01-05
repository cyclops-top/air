# Design: Quit Logic and Download Statistics

## 1. 统计数据模型

在 `src/handlers.rs` 的 `AppState` 中增加以下字段：
```rust
pub struct AppState {
    pub root_path: PathBuf,
    pub total_files: std::sync::atomic::AtomicU64,
    pub total_bytes: std::sync::atomic::AtomicU64,
}
```

## 2. 统计更新逻辑

在 `src/handlers.rs` 的 `handle_request` 函数中：
- 当识别到目标是文件而非目录时：
    - `total_files` 原子加 1。
    - 获取文件大小（从 `metadata.len()`），并累加到 `total_bytes`。
- *注：目录浏览（`list_directory`）不计入统计。*

## 3. 优雅关闭与 'q' 键监听

在 `src/server.rs` 中：
- 使用 `axum::serve(...).with_graceful_shutdown(shutdown_signal(state))`。
- `shutdown_signal` 将使用 `tokio::select!` 监听：
    - `tokio::signal::ctrl_c()`
    - 从 `tokio::io::stdin()` 读取输入，如果是 'q' 则触发关闭。

## 4. 统计输出格式

在 `src/main.rs` 中：
- 在 `server::start` 返回后，从 `state` 中提取数据并格式化输出：
```text
Summary of this session:
  ➜  Files downloaded: 12
  ➜  Total volume:    1.45 MB
```
使用 `src/view.rs` 中现有的 `format_size` 函数进行体积格式化。
