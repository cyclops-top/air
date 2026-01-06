# security Specification

## Purpose
TBD - created by archiving change implement-core-rust. Update Purpose after archive.
## Requirements
### Requirement: 路径沙箱 (Path Sandbox)
系统 MUST 限制所有文件访问在启动时指定的根目录内。

#### Scenario: 根目录访问
启动参数为 `/data`。
用户请求 `/image.png`。
系统解析物理路径 `/data/image.png`。
路径在 `/data` 内，**允许**访问。

#### Scenario: 路径穿越尝试
启动参数为 `/data`。
用户请求 `/../../etc/passwd`。
系统解析路径时必须消除 `..`，如果最终路径指向 `/etc/passwd` (在 `/data` 外)，**拒绝**访问并返回 403。

#### Scenario: 符号链接逃逸
启动参数为 `/data`。
`/data/link_to_root` 是一个指向 `/` 的软链接。
用户请求 `/link_to_root/etc/shadow`。
系统解析软链接后发现目标在 `/data` 外，**拒绝**访问。

### Requirement: 隐藏文件过滤
系统 MUST 默认不展示以点 `.` 开头的文件。

#### Scenario: 列出目录
目录中包含 `file.txt` 和 `.git`。
Web UI 和 JSON 响应中仅包含 `file.txt`，`.git` 被忽略。

