# UI Requirements

## MODIFIED Requirements

### Requirement: 全屏 TUI 仪表盘
TUI 仪表盘 MUST 能够区分并标识分段下载请求。

#### Scenario: 展示 Range 信息
在 TUI 日志窗口中：
- 当一个请求包含 `Range` 头部时：
- 日志条目 MUST 包含该 `Range` 的范围信息（如 `0-1023`）。
- 该信息 SHOULD 紧随 `Path` 动作标签展示。
- 这有助于用户区分针对同一文件的不同分段请求。
