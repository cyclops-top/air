# Server Requirements

## ADDED Requirements

### Requirement: S3 兼容 API 支持
服务器 MUST 提供 S3 标准的对象存储接口，以便专业客户端访问。

#### Scenario: S3 Endpoint 路由
- S3 API MUST 挂载在 `/_s3/` 路径前缀下。
- 该路径 MUST 与现有的 Web UI ( `/` ) 在同一端口共存。
- 这种设计 MUST 能够有效避免 API 路径与用户分享的文件名产生冲突。

#### Scenario: 列表与读取
- 客户端通过 `ListObjectsV2` MUST 能够正确看到本地目录结构。
- 客户端通过 `GetObject` MUST 能够读取文件内容，且 ETag 与 Web 模式下计算的 SHA-256 保持一致。
