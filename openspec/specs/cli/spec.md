# cli Specification

## Purpose
TBD - created by archiving change implement-core-rust. Update Purpose after archive.
## Requirements
### Requirement: 命令行参数解析
程序 MUST 支持指定监听端口，或在未指定时自动选择可用端口。

#### Scenario: 指定端口启动
用户运行 `air --port 9000`。
- 如果 9000 可用，程序 MUST 绑定到该端口。
- 如果 9000 被占用，程序 MUST 报错并退出（以尊重用户的显式指定）。

#### Scenario: 随机端口启动
用户运行 `air`（未提供 `--port`）。
- 程序 MUST 在 10000-65535 范围内随机选择一个端口。
- 如果选中的端口被占用，程序 MUST 自动重新选择并重试，直到成功。
- 最终选定的端口 MUST 准确反映在 Banner 和 TUI 界面中。

### Requirement: 启动横幅 (Startup Banner)
程序启动成功后 MUST 显示有效的网络访问地址，并移除无意义的本地地址信息。

#### Scenario: 组合地址展示
启动后，控制台输出应按以下格式展示地址：
- `➜  Address: http[s]://<hostname>.local:<port> (<lan-ip>)`
- 如果无法获取主机名，则展示：`➜  Address: http[s]://<lan-ip>:<port>`

### Requirement: 运行时日志
所有 HTTP 请求 MUST 实时打印到控制台。

#### Scenario: 正常访问日志
当用户访问 `/file.txt` 且成功时，输出格式如：
`[14:20:01] 192.168.1.15 GET /file.txt 200 (OK) - 50ms`

#### Scenario: 错误访问日志
当用户访问不存在的文件时，输出格式如：
`[14:20:05] 192.168.1.15 GET /missing 404 (Not Found) - 2ms`

### Requirement: 统计摘要输出
程序正常退出后，MUST 在控制台输出本次运行的统计信息。

#### Scenario: 输出下载统计
程序退出后，应按以下格式显示统计：
- `Summary of this session:`
- `  ➜  Files downloaded: <count>`
- `  ➜  Total volume:    <size_with_unit>`

#### Scenario: 统计范围
- 仅统计文件下载请求。
- 目录列表请求 (Directory Listing) MUST 不计入统计。

