# CLI Requirements

## MODIFIED Requirements

### Requirement: 启动横幅 (Startup Banner)
程序启动成功后 MUST 显示服务地址信息以及退出指令提示。

#### Scenario: 显示退出提示
启动后，控制台输出除包含地址外，应包含：
- `Hit CTRL-C or press 'q' to stop the server`

## ADDED Requirements

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
