## 📄 Air 项目功能扩展：局域网服务发现 (Service Discovery)

### 1. 背景与目标

当前 `air` 是一个轻量级的文件分享工具。为了提升用户体验，需增加“服务发现”功能。

* **服务端**：在启动文件服务的同时，自动通过 UDP 广播自身信息（IP、端口、主机名）。
* **客户端**：新增 `discover` 模式，监听广播并列出局域网内可用的 Air 服务节点。

### 2. 技术规范 (Protocol Specs)

#### 2.1 通信协议

* **传输层**：UDP Broadcast (IPv4)
* **数据格式**：JSON (UTF-8)

#### 2.2 端口策略 (Multi-Port Strategy)

为了防止端口冲突，采用“主备端口组”策略。建议选用以下 **安全号段**（避开系统临时端口）：

```rust
const DISCOVERY_PORTS: [u16; 3] = [29888, 29889, 29890];

```

#### 2.3 数据载荷 (Payload Structure)

广播消息必须包含以下字段：

```json
{
  "name": "Jack's MacBook Pro",  // 主机名
  "ip": "192.168.1.5",           // 局域网 IP
  "port": 9000,                  // HTTP 服务端口
  "scheme": "http"               // "http" 或 "https"
}

```

### 3. 实现逻辑详解

#### 3.1 依赖库变更 (`Cargo.toml`)

需要引入以下 crate 以支持序列化、底层 Socket 控制和系统信息获取：

* `serde` & `serde_json`: 处理 JSON。
* `socket2`: 处理底层 Socket 选项（特别是 macOS 的端口复用）。
* `local-ip-address`: 获取本机局域网 IP。
* `hostname`: 获取设备名称。

#### 3.2 服务端逻辑 (Broadcaster)

* **触发时机**：在 `main` 函数启动 HTTP Server 之前，通过 `tokio::spawn` 启动后台任务。
* **Socket 配置**：
* 绑定地址：`0.0.0.0:0` (系统随机分配发送端口)。
* 选项：开启 `SO_BROADCAST`。


* **发送逻辑**：
* 进入无限循环。
* 每隔 **3秒**。
* 遍历 `DISCOVERY_PORTS` 数组，向每个端口的 `255.255.255.255:<port>` 发送 JSON 数据包。
* 忽略单次发送失败（Fire and forget）。



#### 3.3 客户端逻辑 (Listener)

* **触发时机**：新增 CLI 子命令 `air discover` (或参数 `--discover -d`)。
* **Socket 配置 (关键 - macOS 兼容性)**：
* 使用 `socket2` 创建 Socket。
* **必须开启** `SO_REUSEADDR`。
* **必须开启** `SO_REUSEPORT` (macOS/iOS 必需，用于解决多进程绑定冲突)。
* 绑定地址：`0.0.0.0:<port>`。


* **监听逻辑**：
* 遍历 `DISCOVERY_PORTS`，尝试绑定其中 **任意一个** 可用端口。
* 一旦绑定成功，进入 `recv_from` 循环。
* 收到数据后反序列化 JSON，并在终端格式化打印服务信息。



### 4. 代码结构建议

建议新建模块 `src/discovery.rs`，并在 `src/main.rs` 中调用。

```rust
// src/discovery.rs 伪代码结构

#[derive(Serialize, Deserialize)]
pub struct DiscoveryMsg { ... }

// 服务端入口
pub async fn start_broadcast(http_port: u16, https: bool) -> anyhow::Result<()> {
    // 1. 获取 local_ip 和 hostname
    // 2. 启动 tokio task 循环发送
}

// 客户端入口
pub async fn run_discovery() -> anyhow::Result<()> {
    // 1. 尝试绑定端口 (使用 socket2 设置 REUSEPORT)
    // 2. 循环接收并打印
}

```

### 5. 验收标准

1. **多路发送**：服务端启动后，使用 Wireshark 或 `nc` 应能抓到发往 29888-29890 的 UDP 包。
2. **macOS 兼容**：在 macOS (Apple Silicon) 上，允许开启多个终端同时运行 `air discover`，互不报错（验证 `SO_REUSEPORT`）。
3. **信息准确**：客户端显示的 IP 和端口应能直接点击访问。
