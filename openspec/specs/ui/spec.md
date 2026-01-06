# ui Specification

## Purpose
TBD - created by archiving change implement-tui-dashboard. Update Purpose after archive.
## Requirements
### Requirement: 全屏 TUI 仪表盘
TUI 仪表盘 MUST 提供高质量、易读且已解码的实时日志路径显示。

#### Scenario: 路径展示解码
在 TUI 日志窗口中：
- 路径字符串 MUST 经过 URL 解码处理。
- 例如：原始请求路径 `/My%20Folder` MUST 显示为 `/My Folder`。

### Requirement: 退出后统计展示
程序退出后的统计摘要 MUST 包含总运行时长。

#### Scenario: 打印总运行时长
程序关闭后，控制台最后输出的 `Summary of this session` MUST 包含：
- `  ➜  Total uptime:    <duration>`

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

### Requirement: 目录列表页面路径展示
Web UI MUST 使用可点击的面包屑（Breadcrumbs）展示当前路径，取代静态文本。

#### Scenario: 根目录展示
当用户位于 `/` 时：
- 面包屑显示为 `Home`。
- `Home` 可点击（指向 `/`）。

#### Scenario: 深层目录展示
当用户位于 `/Movies/Action` 时：
- 面包屑显示为 `Home / Movies / Action`。
- `Home` 指向 `/`。
- `Movies` 指向 `/Movies`。
- `Action` 指向 `/Movies/Action` 或作为当前位置展示。

#### Scenario: 面包屑交互
用户点击面包屑中的 `Movies`：
- 浏览器跳转到 `http://<ip>:<port>/Movies`。
- 页面展示 `/Movies` 目录的内容。

