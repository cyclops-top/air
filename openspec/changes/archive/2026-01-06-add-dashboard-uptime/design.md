# Design: Dashboard Uptime Timer

## 1. 启动时间记录

在 `src/handlers.rs` 的 `Stats` 结构体中增加 `start_time` 字段：
```rust
pub struct Stats {
    pub total_files: AtomicU64,
    pub total_bytes: AtomicU64,
    pub logs: Mutex<VecDeque<String>>,
    pub start_time: std::time::Instant,
}
```
由于 `Instant::now()` 在 `Default` 实现中调用即可代表 `Stats` 创建的时间（即服务器启动时间）。

## 2. 持续时间计算

在 `src/dashboard.rs` 的渲染循环中，通过 `app_state.stats.start_time.elapsed()` 计算当前运行时长。

## 3. 格式化逻辑

实现一个简单的辅助函数将 `Duration` 格式化为 `HH:MM:SS` 或更友好的字符串（如 `1h 23m 45s`）。

## 4. UI 布局

在 `System Status` 区域增加一行或在现有的 `[Stats]` 行中插入运行时长：
`[Stats] Files: 12 | Volume: 4.5 MB | Uptime: 00:15:30`
