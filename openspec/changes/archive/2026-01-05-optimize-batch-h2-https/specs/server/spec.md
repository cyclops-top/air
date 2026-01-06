# Server Requirements

## ADDED Requirements

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
