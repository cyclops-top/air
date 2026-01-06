# Server Requirements

## ADDED Requirements

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
