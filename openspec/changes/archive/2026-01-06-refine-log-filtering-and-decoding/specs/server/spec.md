# Server Requirements

## MODIFIED Requirements

### Requirement: 语义化请求日志
服务器 MUST 仅针对核心用户行为记录语义化日志。

#### Scenario: 过滤 HEAD 请求
当服务器接收到 `HEAD` 类型的请求时：
- 该请求 MUST 不会被记录到 TUI 日志窗口中。

#### Scenario: 仅限 GET 请求记录
- 只有当请求方法为 `GET` 时，系统 MUST 才会尝试记录 `OPEN DIR` 或 `DOWNLOAD` 等语义化日志。
