# CLI Requirements

## ADDED Requirements

### Requirement: 命令行参数解析
程序 MUST 支持指定共享目录路径和监听端口。

#### Scenario: 默认启动
当用户仅运行 `air` 时：
- 监听端口默认为 `8000`。
- 共享目录默认为当前工作目录 (`.`)。

#### Scenario: 指定参数启动
当用户运行 `air /var/www --port 3000` 时：
- 共享目录为 `/var/www`。
- 监听端口为 `3000`。

#### Scenario: 端口占用处理
当指定端口被占用时：
- (可选) 自动递增端口直到找到可用端口，或直接报错退出（根据 Spec 2.1 描述"自动递增或随机"）。
- *注：本期实现优先支持指定端口，若冲突可报错，后续优化自动递增。*

### Requirement: 启动横幅 (Startup Banner)
程序启动成功后 MUST 显示服务地址信息。

#### Scenario: 显示局域网 IP
启动后，控制台输出应包含：
- 本地地址: `http://localhost:<port>`
- 局域网地址: `http://<lan-ip>:<port>`
- 安全提示: `Security Check: SANDBOX ENABLED 🔒`

### Requirement: 运行时日志
所有 HTTP 请求 MUST 实时打印到控制台。

#### Scenario: 正常访问日志
当用户访问 `/file.txt` 且成功时，输出格式如：
`[14:20:01] 192.168.1.15 GET /file.txt 200 (OK) - 50ms`

#### Scenario: 错误访问日志
当用户访问不存在的文件时，输出格式如：
`[14:20:05] 192.168.1.15 GET /missing 404 (Not Found) - 2ms`