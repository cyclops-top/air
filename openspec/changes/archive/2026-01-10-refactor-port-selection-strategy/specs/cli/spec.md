# CLI Requirements

## MODIFIED Requirements

### Requirement: 命令行参数解析
程序 MUST 支持指定监听端口，或在未指定时智能选择可用端口。

#### Scenario: 随机端口启动
用户运行 `air`（未提供 `--port`）。
- 程序 MUST 首先尝试绑定到一个默认的不常用端口（如 `9568`）。
- 如果该默认端口已被占用，程序 MUST 在 10000-65535 范围内重新随机选择一个端口并重试，直到成功绑定。
- 最终选定的端口 MUST 准确反映在 Banner 和 TUI 界面中。
