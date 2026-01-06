# Server Requirements

## MODIFIED Requirements

### Requirement: Favicon 支持
服务器 MUST 提供网站图标（Favicon）支持，以避免浏览器默认请求产生错误。

#### Scenario: 嵌入式资源服务
当浏览器请求 `/favicon.ico` 时：
- 服务器 MUST 返回在**编译时嵌入**在二进制文件中的 SVG 内容。
- 即使运行环境下不存在 `docs/favicon.svg` 文件，该请求也 MUST 成功。
- 响应头 MUST 包含正确的 Content-Type (`image/svg+xml`)。
- 该请求 MUST 绕过共享目录的路径沙箱检查。
