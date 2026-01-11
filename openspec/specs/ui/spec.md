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
- `Movies` 指向 `/Movies/`。
- `Action` 指向 `/Movies/Action/`。

#### Scenario: 面包屑交互
用户点击面包屑中的 `Movies`：
- 浏览器跳转到 `http://<ip>:<port>/Movies/`。
- 页面展示 `/Movies` 目录的内容。

### Requirement: Cyber-Space 视觉风格
Web UI MUST 采用 Cyber-Space 风格设计，包含玻璃拟态 (Glassmorphism) 和动态发光效果。

#### Scenario: 玻璃拟态卡片
目录列表 MUST 展示在一个具有背景模糊 (`backdrop-blur`)、半透明边框和微弱阴影的卡片容器中。

#### Scenario: 状态指示器
页面顶部 MUST 包含一个 "System Online" 指示器，包含绿色呼吸灯效果。

#### Scenario: 发光字体效果
当前路径的面包屑末尾项和鼠标悬停时的文件名 MUST 具有青色 (`#00f7ff`) 的发光 (`glow`) 效果。

### Requirement: 增强型文件列表
文件列表 MUST 提供更丰富的视觉层次和交互反馈。

#### Scenario: 响应式表格布局
文件列表 MUST 使用表格形式展示，包含：
- **Filename**: 文件名及对应图标。
- **Size**: 文件大小。
- **Last Sync**: 易读的最后修改时间（格式如 `2024.10.24 // 14:02`）。
- **Access**: 右侧对齐的下载操作按钮。

#### Scenario: 图标化文件类型
不同文件类型 SHOULD 使用不同的图标进行区分（例如：文件夹、文档、日志、地图等）。

#### Scenario: 交互反馈
文件行在鼠标悬停时 MUST 改变背景颜色，且文件名发光，下载图标放大。

### Requirement: 主题切换与持久化 (Theme Switching)
Web UI MUST 支持在浅色 (Light) 和深色 (Dark) 风格之间进行切换，并记住用户的选择。

#### Scenario: 初始加载
- 当用户首次访问页面时，系统 MUST 根据其浏览器/系统的偏好设置（或默认为深色）应用相应的主题。
- 如果用户之前在同一浏览器中设置过主题，系统 MUST 优先应用该持久化的主题。

#### Scenario: 切换交互
- 页面顶部 Header MUST 包含一个主题切换按钮。
- 点击该按钮 MUST 立即改变页面的视觉风格，并更新 `localStorage` 以保存用户偏好。

### Requirement: macOS 玻璃拟态风格 (macOS Glassmorphism)
Web UI MUST 采用 macOS 风格的视觉设计，强调通透感和圆润感。

#### Scenario: Bokeh 背景效果
页面背景 MUST 包含若干具有高斯模糊效果的彩色光斑（Bokeh），且光斑颜色在深浅主题下具有不同的明度和透明度。

#### Scenario: 增强型图标胶囊
文件列表中的图标 MUST 被包裹在一个具有背景色的圆角胶囊容器中：
- 文件夹、文档、日志、媒体等不同类型 MUST 使用不同的胶囊背景色（如蓝色、绿色、紫色等）。
- 胶囊背景色在深浅主题下应自动调整明度。

