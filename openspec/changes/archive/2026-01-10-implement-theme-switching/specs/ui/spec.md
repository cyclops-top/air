# Spec Delta: macOS-style Theme Switching

## ADDED Requirements

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
