# Spec Delta: Memory-Mapped File Sharing

## ADDED Requirements

### Requirement: 高效文件共享 (Memory Mapping)
服务器 MUST 使用内存映射 (`mmap`) 技术来提升高并发下的文件读取性能。

#### Scenario: 共享内存映射
当多个客户端并发请求同一个大文件时：
- 服务器 MUST 仅为该文件创建一个内存映射实例。
- 所有请求 MUST 共享该映射中的数据，减少内存占用和系统调用。

#### Scenario: 引用计数与自动释放
- 服务器 MUST 跟踪每个内存映射的使用情况。
- 当最后一个持有该映射的 HTTP 响应完成发送后，服务器 MUST 及时释放（关闭）该内存映射以回收系统资源。

#### Scenario: 0 拷贝传输
- 服务器 SHOULD 尽可能实现“零拷贝”传输，即直接将内存映射的页面交给网络协议栈，避免在应用层进行数据拷贝。

#### Scenario: 并发创建保护
- 当高并发请求一个尚未映射的文件时，服务器 MUST 确保只执行一次映射操作，避免竞态条件导致重复映射。
