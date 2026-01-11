# Spec Delta: File Modification Time Display

## MODIFIED Requirements

### Requirement: 目录列表服务
当请求路径指向一个目录时，服务器 MUST 返回该目录的内容列表。

#### Scenario: HTML 响应
用户请求 `/photos/` 且 Header `Accept: text/html` (浏览器默认行为)。
服务器返回渲染好的 HTML 页面。
页面包含：
- 面包屑导航 `Home > photos`。
- 文件列表，区分文件夹和文件图标。
- 文件大小展示。
- **文件最后修改时间展示**。
