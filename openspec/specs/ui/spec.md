# ui Specification

## Purpose
TBD - created by archiving change implement-tui-dashboard. Update Purpose after archive.
## Requirements
### Requirement: 全屏 TUI 仪表盘
TUI 仪表盘 MUST 实时显示网络访问地址。

#### Scenario: 仪表盘布局更新
当进入 TUI 模式时：
- 屏幕顶部 MUST 移除 `Local` 地址行。
- MUST 显示统一的 `Network Address`，包含主机名和局域网 IP。

### Requirement: 退出后统计展示
在 TUI 模式结束后，程序 MUST 将运行统计数据打印到常规标准输出。

#### Scenario: 打印摘要
程序关闭后，控制台最后输出的内容 MUST 包含：
- 总计下载文件数。
- 总计下载字节量（格式化后的）。

### Requirement: TUI 二维码展示
TUI 仪表盘 MUST 在显著位置（如顶部 Header）展示一个二维码，用于快速访问。

#### Scenario: 动态二维码生成
当程序在 TUI 模式下启动时：
- 程序 MUST 根据当前的 `Network` 访问地址生成对应的二维码。
- 二维码 MUST 使用终端可显示的 Unicode 字符渲染。

#### Scenario: 布局一致性
- 二维码 SHOULD 与 `System Status` 文本并排显示。
- 即使窗口高度受限，二维码也 MUST 尽可能保持显示，以便手机扫码。

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

