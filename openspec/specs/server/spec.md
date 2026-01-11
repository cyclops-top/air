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

#### Scenario: 强制尾随斜杠
当请求路径指向一个目录但末尾没有 `/` 时（如 `/photos`）：
- 服务器 MUST 返回 `301 Moved Permanently` 重定向到带斜杠的路径（如 `/photos/`）。
- 如果原请求包含查询参数，重定向 MUST 保留这些参数。

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
服务器 MUST 返回 `404 Not Found`。

#### Scenario: 路径越界保护
当请求路径尝试通过 `..` 等手段访问根目录以外的文件时：
- 服务器 MUST 返回 `403 Forbidden`。

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

### Requirement: HTTP 实例摘要 (Digest)
服务器 MUST 为下载的文件提供应用层完整性校验。

#### Scenario: 响应包含 SHA-256
当用户成功请求一个文件时：
- 响应头 MUST 包含 `Digest` 字段。
- 格式 MUST 符合 RFC 3230/5843 标准（如 `SHA-256=...`）。
- 摘要值 MUST 是文件内容的有效哈希。

#### Scenario: 缓存与自动失效
- 服务器 SHOULD 在内存中缓存已计算的摘要以提升性能。
- 如果文件的大小或最后修改时间发生变化，服务器 MUST 重新计算摘要，严禁返回过期的哈希值。

#### Scenario: 分段下载中的摘要
- 当响应为 `206 Partial Content` (Range 请求) 时，`Digest` 头部 MUST 依然包含**完整文件**的摘要，而非仅该分段的摘要。

### Requirement: 语义化请求日志
服务器 MUST 仅针对核心用户行为记录语义化日志。

#### Scenario: 过滤 HEAD 请求
当服务器接收到 `HEAD` 类型的请求时：
- 该请求 MUST 不会被记录到 TUI 日志窗口中。

#### Scenario: 仅限 GET 请求记录
- 只有当请求方法为 `GET` 时，系统 MUST 才会尝试记录 `OPEN DIR` 或 `DOWNLOAD` 等语义化日志。

### Requirement: 高效缓存与校验
服务器 MUST 支持 HTTP 标准的缓存协商机制。

#### Scenario: ETag 生成与验证
当请求文件时：
- 服务器 MUST 返回 `ETag` 头部，值由文件的 SHA-256 摘要生成。
- 如果客户端发送了匹配的 `If-None-Match` 头部：
    - 服务器 MUST 返回 `304 Not Modified`。
    - 响应体 MUST 为空。

#### Scenario: 续传一致性校验
在使用 `Range` 请求时：
- 服务器 MUST 始终返回 `ETag` 头部。
- 客户端可以使用该 ETag 确保多个分段属于同一个文件版本。

### Requirement: 高效文件共享 (Memory Mapping)
服务器 MUST 使用内存映射 (`mmap`) 技术来提升高并发下的文件读取性能。

#### Scenario: 共享内存映射
当多个客户端并发请求同一个大文件时：
- 服务器 MUST 仅为该文件创建一个内存映射实例。
- 所有请求 MUST 共享该映射中的数据，减少内存占用和系统调用。

#### Scenario: 引用计数与自动释放
- 服务器 MUST 跟踪每个内存映射的使用情况。
- 当最后一个持有该映射的 HTTP 响应完成发送后，服务器 MUST 及时释放（关闭）该内存映射以回收系统资源。

#### Scenario: 0 拷贝传输
- 服务器 SHOULD 尽可能实现“零拷贝”传输，即直接将内存映射的页面交给网络协议栈，避免在应用层进行数据拷贝。

#### Scenario: 并发创建保护
- 当高并发请求一个尚未映射的文件时，服务器 MUST 确保只执行一次映射操作，避免竞态条件导致重复映射。

