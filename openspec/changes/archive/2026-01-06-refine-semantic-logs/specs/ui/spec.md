# UI Requirements

## MODIFIED Requirements

### Requirement: 全屏 TUI 仪表盘
TUI 仪表盘 MUST 提供高质量、易读且色彩丰富的实时日志显示。

#### Scenario: 日志对齐与格式
在 TUI 日志窗口中：
- 每一行日志 MUST 按照 `[时间] [IP] [动作] [耗时] [路径]` 的顺序展示。
- 前四个字段 MUST 保持固定宽度以实现垂直对齐。
- 路径字段 MUST 根据剩余空间自动截断，并以 `...` 结尾，严禁换行。

#### Scenario: 日志色彩与语义
- 不同的动作标签（如 `OPEN DIR` 和 `DOWNLOAD`）MUST 使用不同的颜色区分。
- 请求失败时，路径前缀 MUST 包含 `✖` 符号，且路径文本 MUST 显示为红色。

#### Scenario: 记录过滤
- 内部静默请求（如 `FAVICON`）MUST 不出现在日志窗口中。
