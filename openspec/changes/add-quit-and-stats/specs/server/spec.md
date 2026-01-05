# Server Requirements

## ADDED Requirements

### Requirement: 优雅关闭支持
服务器 MUST 支持优雅关闭。

#### Scenario: 监听退出指令
服务器在运行过程中 MUST 监听以下事件以触发优雅关闭：
- 系统的 `SIGINT` (Ctrl-C) 信号。
- 标准输入 (stdin) 中的 `q` 字符（及回车）。
- 优雅关闭期间应允许当前正在处理的请求完成。
