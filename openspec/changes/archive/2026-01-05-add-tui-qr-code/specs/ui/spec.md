# UI Requirements

## ADDED Requirements

### Requirement: TUI 二维码展示
TUI 仪表盘 MUST 在显著位置（如顶部 Header）展示一个二维码，用于快速访问。

#### Scenario: 动态二维码生成
当程序在 TUI 模式下启动时：
- 程序 MUST 根据当前的 `Network` 访问地址生成对应的二维码。
- 二维码 MUST 使用终端可显示的 Unicode 字符渲染。

#### Scenario: 布局一致性
- 二维码 SHOULD 与 `System Status` 文本并排显示。
- 即使窗口高度受限，二维码也 MUST 尽可能保持显示，以便手机扫码。
