# UI Requirements

## MODIFIED Requirements

### Requirement: TUI 二维码展示
TUI 仪表盘 MUST 根据终端能力智能展示高清二维码。

#### Scenario: 基于图形协议的渲染
当终端支持 Sixel, Kitty 或 iTerm2 图形协议时：
- 系统 MUST 将二维码渲染为像素级图片。
- 二维码 MUST 清晰、无形变且可被扫描。

#### Scenario: 自动隐藏
当终端不支持任何已知图形协议时：
- 系统 MUST 隐藏二维码区域。
- 仪表盘布局 MUST 自动调整，将原本留给二维码的空间用于展示其他系统信息。
