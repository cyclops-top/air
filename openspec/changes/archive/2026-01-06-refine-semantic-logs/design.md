# Design: Refined Semantic Logging

## 1. 结构化日志模型 (LogEntry)

在 `src/handlers.rs` 中定义：
```rust
pub struct LogEntry {
    pub time: String,
    pub ip: String,
    pub action: LogAction,
    pub duration: std::time::Duration,
    pub path: String,
    pub is_success: bool,
}
```

## 2. 布局与列宽

| 列名 | 宽度 | 样式 |
| :--- | :--- | :--- |
| 时间 | 10 | 灰色 |
| IP | 16 | 默认 |
| 动作 | 10 | 颜色 (Green/Blue) |
| 耗时 | 10 | 默认 |
| 路径 | 剩余 | 默认 / 红色 + ✖ |

## 3. 截断逻辑

在 `src/dashboard.rs` 中，根据当前渲染区域的宽度，动态截断 `LogEntry.path`。
`truncated_path = if path.len() > available { format!("{}...", &path[..available-3]) } else { path }`

## 4. Middleware 更新

修改 `src/logger.rs` 中的 `log_request`：
- 识别 `LogAction::Favicon` 并直接返回，不存入队列。
- 收集元数据并构造 `LogEntry` 存入 `stats.logs`。
