# CLI Requirements

## MODIFIED Requirements

### Requirement: 启动横幅 (Startup Banner)
程序启动成功后 MUST 显示有效的网络访问地址，并移除无意义的本地地址信息。

#### Scenario: 组合地址展示
启动后，控制台输出应按以下格式展示地址：
- `➜  Address: http[s]://<hostname>.local:<port> (<lan-ip>)`
- 如果无法获取主机名，则展示：`➜  Address: http[s]://<lan-ip>:<port>`
