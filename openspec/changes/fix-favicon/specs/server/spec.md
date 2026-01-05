# Server Requirements

## ADDED Requirements

### Requirement: Favicon 支持
服务器 MUST 提供网站图标（Favicon）支持，以避免浏览器默认请求产生 403 或 404 错误。

#### Scenario: 请求 Favicon
当浏览器请求 `/favicon.ico` 时：
- 服务器 MUST 返回位于项目 `docs/favicon.svg` 的文件内容。
- 响应头 MUST 包含正确的 Content-Type (如 `image/svg+xml`)。
- 该请求 MUST 绕过共享目录的路径沙箱检查。

### Requirement: HTML 图标引用
Web UI 的 HTML 页面 MUST 显式引用网站图标。

#### Scenario: HTML Head 包含图标
生成的 HTML 源代码中 MUST 包含 `<link rel='icon' ...>` 标签，指向图标路由。