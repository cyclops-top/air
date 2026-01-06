# Server Requirements

## ADDED Requirements

### Requirement: 语义化请求日志
服务器 MUST 在日志记录中使用语义化的动作描述，而非单纯的 HTTP 方法。

#### Scenario: 记录目录访问
当用户成功访问一个文件夹并列出内容时：
- 日志中的动作标签 MUST 为 `OPEN DIR` (或类似的语义化词汇)。

#### Scenario: 记录文件下载
当用户成功请求并开始下载一个文件时：
- 日志中的动作标签 MUST 为 `DOWNLOAD` (或类似的语义化词汇)。

#### Scenario: 兼容性处理
对于非目录或文件下载的特殊请求（如 Favicon 或 404 错误）：
- 系统 SHOULD 尽可能使用相应的语义化标签（如 `FAVICON` 或保持 `GET`）。
