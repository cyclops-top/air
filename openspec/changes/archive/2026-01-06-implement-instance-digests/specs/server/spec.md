# Server Requirements

## ADDED Requirements

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
