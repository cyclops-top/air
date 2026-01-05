# Server Requirements

## ADDED Requirements

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