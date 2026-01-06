# CLI Requirements

## MODIFIED Requirements

### Requirement: 命令行参数解析
程序 MUST 支持分别指定 Web UI 和 S3 API 的监听端口。

#### Scenario: 指定 S3 端口
用户运行 `air --s3-port 9001`。
- S3 服务 MUST 绑定到 9001 端口（若可用）。

#### Scenario: 随机 S3 端口
用户未提供 `--s3-port`。
- 程序 MUST 自动选择一个不冲突的可用端口作为 S3 入口。

### Requirement: 启动横幅 (Startup Banner)
程序启动成功后 MUST 同时显示 Web UI 和 S3 API 的访问地址。

#### Scenario: 双地址输出
控制台输出 MUST 包含类似以下内容：
- `➜  Web UI: http://...:<port>`
- `➜  S3 API: http://...:<s3_port>`
