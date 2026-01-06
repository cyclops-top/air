# CLI Requirements

## MODIFIED Requirements

### Requirement: 命令行参数解析
程序 MUST 支持指定监听端口，或在未指定时自动选择可用端口。

#### Scenario: 指定端口启动
用户运行 `air --port 9000`。
- 如果 9000 可用，程序 MUST 绑定到该端口。
- 如果 9000 被占用，程序 MUST 报错并退出（以尊重用户的显式指定）。

#### Scenario: 随机端口启动
用户运行 `air`（未提供 `--port`）。
- 程序 MUST 在 10000-65535 范围内随机选择一个端口。
- 如果选中的端口被占用，程序 MUST 自动重新选择并重试，直到成功。
- 最终选定的端口 MUST 准确反映在 Banner 和 TUI 界面中。
