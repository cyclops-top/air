# Server Requirements

## ADDED Requirements

### Requirement: 独立 S3 服务
服务器 MUST 提供运行在独立端口上的 S3 兼容接口。

#### Scenario: 独立端口监听
- S3 服务 MUST 使用与 Web UI 不同的 TCP 端口进行监听。
- 这种设计 MUST 确保文件系统中的任何文件路径（包括名为 `_s3` 的文件）都能被正常通过 Web UI 访问，不受 API 路径干扰。

#### Scenario: 协议一致性
- 客户端通过 S3 API 读取的文件内容、ETag 和 Range 支持 MUST 与 Web UI 保持完全一致。
