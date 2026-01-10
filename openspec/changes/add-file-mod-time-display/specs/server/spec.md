# Spec Delta: Cache Invalidation

## MODIFIED Requirements

### Requirement: HTTP 实例摘要 (Digest)
服务器 MUST 为下载的文件提供应用层完整性校验。

#### Scenario: 响应包含 SHA-256
当用户成功请求一个文件时：
- 响应头 MUST 包含 `Digest` 字段。
- 格式 MUST 符合 RFC 3230/5843 标准（如 `SHA-256=...`）。
- 摘要值 MUST 是文件内容的有效哈希。

#### Scenario: 缓存与自动失效
- 服务器 SHOULD 在内存中缓存已计算的摘要以提升性能。
- **如果文件的最后修改时间 (mtime) 或大小 (size) 发生变化，服务器 MUST 立即失效缓存，并重新计算摘要，严禁返回过期的哈希值**。
