# Design: Refactor Address Display

## 1. 主机名获取

使用 `hostname` crate 获取系统的主机名。为了兼容性，通常在局域网中可以通过 `<hostname>.local` 来访问（mDNS/Bonjour）。

逻辑：
1. `hostname::get()` 获取 `OsString`。
2. 转换为 `String`。
3. 组合展示：`{hostname}.local ({ip})` 或直接 `{hostname} ({ip})`。

## 2. 界面变更

### 2.1 启动 Banner (main.rs)
旧：
```text
  ➜  Local:   http://localhost:8000
  ➜  Network: http://192.168.1.5:8000
```
新：
```text
  ➜  Address: http://justin-macbook.local:8000 (192.168.1.5)
```

### 2.2 TUI 仪表盘 (dashboard.rs)
将原有的两行合并为一行，或者使用更清晰的标签。
```text
Network Address: https://justin-mbp.local:45153 (192.168.3.116)
```

## 3. 跨平台考量
`hostname` crate 支持 Unix 和 Windows。如果获取失败，程序应有 fallback 逻辑（仅显示 IP）。
