# Spec Delta: Cyber-Space UI Implementation

## ADDED Requirements

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
