# UI Requirements

## MODIFIED Requirements

### Requirement: 全屏 TUI 仪表盘
TUI 仪表盘 MUST 提供高质量、易读且已解码的实时日志路径显示。

#### Scenario: 路径展示解码
在 TUI 日志窗口中：
- 路径字符串 MUST 经过 URL 解码处理。
- 例如：原始请求路径 `/My%20Folder` MUST 显示为 `/My Folder`。
