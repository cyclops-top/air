# server Specification

## Purpose
TBD - created by archiving change implement-core-rust. Update Purpose after archive.
## Requirements
### Requirement: 目录列表服务
当请求路径指向一个目录时，服务器 MUST 返回该目录的内容列表。

#### Scenario: JSON 响应
用户请求 `/photos/` 且 Header `Accept: application/json`。
服务器返回 200 OK 和 JSON 数据，符合 OpenAPI Schema `DirectoryListing`。包含文件名、大小、修改时间、是否为目录。

#### Scenario: HTML 响应
用户请求 `/photos/` 且 Header `Accept: text/html` (浏览器默认行为)。
服务器返回渲染好的 HTML 页面。
页面包含：
- 面包屑导航 `Home > photos`。
- 文件列表，区分文件夹和文件图标。
- 文件大小展示。

### Requirement: 文件下载服务
当请求路径指向一个文件时，服务器 MUST 提供文件内容。

#### Scenario: 完整下载
用户请求 `/video.mp4`。
服务器返回 200 OK 和完整文件流。
设置 `Content-Length` 和 `Content-Type`。

#### Scenario: 断点续传 (Range Request)
用户请求 `/video.mp4` 且 Header `Range: bytes=0-1023`。
服务器返回 206 Partial Content。
Header `Content-Range: bytes 0-1023/文件总大小`。
Body 仅包含前 1024 字节。

#### Scenario: 文件未找到
用户请求不存在的路径。
服务器返回 404 Not Found。

### Requirement: Favicon 支持
服务器 MUST 提供网站图标（Favicon）支持，以避免浏览器默认请求产生错误。

#### Scenario: 嵌入式资源服务
当浏览器请求 `/favicon.ico` 时：
- 服务器 MUST 返回在**编译时嵌入**在二进制文件中的 SVG 内容。
- 即使运行环境下不存在 `docs/favicon.svg` 文件，该请求也 MUST 成功。
- 响应头 MUST 包含正确的 Content-Type (`image/svg+xml`)。
- 该请求 MUST 绕过共享目录的路径沙箱检查。

### Requirement: HTML 图标引用
Web UI 的 HTML 页面 MUST 显式引用网站图标。

#### Scenario: HTML Head 包含图标
生成的 HTML 源代码中 MUST 包含 `<link rel='icon' ...>` 标签，指向图标路由。

### Requirement: 优雅关闭支持
服务器 MUST 支持优雅关闭。

#### Scenario: 监听退出指令
服务器在运行过程中 MUST 监听以下事件以触发优雅关闭：
- 系统的 `SIGINT` (Ctrl-C) 信号。
- 标准输入 (stdin) 中的 `q` 字符（及回车）。
- 优雅关闭期间应允许当前正在处理的请求完成。

### Requirement: HTTPS 与 HTTP/2 支持
当开启 HTTPS 模式时，服务器 MUST 支持 HTTP/2 以优化批量请求性能。

#### Scenario: 自动生成证书
开启 HTTPS 时：
- 服务器 MUST 自动生成针对当前局域网 IP 的自签名证书。
- 证书 MUST 包含 `Subject Alternative Name (SAN)` 扩展以覆盖局域网 IP。

#### Scenario: HTTP/2 多路复用
在 HTTPS 连接下：
- 服务器 MUST 开启 HTTP/2 特性。
- 浏览器在处理“批量请求”（如并发加载多个图标或小文件）时应能共享单一连接。

