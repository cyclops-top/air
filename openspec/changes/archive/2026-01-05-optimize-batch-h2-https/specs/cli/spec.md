# CLI Requirements

## MODIFIED Requirements

### Requirement: 命令行参数解析
程序 MUST 支持指定共享目录路径、监听端口以及是否开启 HTTPS。

#### Scenario: 开启 HTTPS
用户运行 `air --https`。
程序 MUST 在启动时生成自签名证书，并以 HTTPS 协议启动服务。

#### Scenario: 默认 HTTP
用户运行 `air`（不带 `--https`）。
程序 MUST 以传统的 HTTP 协议启动服务。
