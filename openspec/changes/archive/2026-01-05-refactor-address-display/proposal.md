# Proposal: 优化地址显示：移除 Local 地址并引入主机名

## Why
目前 `air` 在启动时和 TUI 界面中都会显示 `Local: http://localhost:<port>`。由于该工具主要用于局域网分享，`localhost` 地址对其他设备访问没有指导意义。同时，直接展示局域网内的计算机名（Hostname）比纯 IP 地址更具识别性。

## What Changes
- **移除 Local 地址**: 在所有界面（Banner 和 TUI）中不再显示 `localhost` 地址。
- **引入主机名显示**: 
    - 引入 `hostname` crate 获取计算机名。
    - 将地址展示格式优化为：`http://<hostname>:<port> (<ip>)`。
    - 如果获取主机名失败，则仅显示 `http://<ip>:<port>`。
- **TUI 适配**: 更新仪表盘布局，合并原有的 Local/Network 行，以更简洁的方式展示网络访问信息。

## Impact
- **交互简洁性**: 减少冗余信息，直观展示外部设备可用的连接方式。
- **易读性**: 用户可以更清晰地看到自己的计算机名和对应的局域网 IP。
