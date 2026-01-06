# UI Requirements

## MODIFIED Requirements

### Requirement: 全屏 TUI 仪表盘
TUI 仪表盘 MUST 实时显示服务运行时长。

#### Scenario: 运行时长展示
在 TUI 模式下：
- 顶部 Header 区域 MUST 包含 `Uptime` 字段。
- 运行时长 MUST 随时间实时更新。

### Requirement: 退出后统计展示
程序退出后的统计摘要 MUST 包含总运行时长。

#### Scenario: 打印总运行时长
程序关闭后，控制台最后输出的 `Summary of this session` MUST 包含：
- `  ➜  Total uptime:    <duration>`
