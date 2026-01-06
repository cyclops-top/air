# Tasks: Implement Semantic Logging

1.  **定义日志模型**
    - [x] 在 `src/logger.rs` 中定义 `LogAction` 枚举。
    - [x] 验证：编译通过。

2.  **更新请求处理器 (Handlers)**
    - [x] 在 `src/handlers.rs::handle_request` 中，为目录响应注入 `LogAction::OpenDir`。
    - [x] 为文件下载响应注入 `LogAction::DownloadFile`。
    - [x] 在 `src/handlers.rs::favicon` 中注入 `LogAction::Favicon`。
    - [x] 验证：响应扩展注入逻辑无语法错误。

3.  **重构日志中间件 (Middleware)**
    - [x] 修改 `src/logger.rs::log_request`，从响应中提取 `LogAction`。
    - [x] 根据 Action 转换展示文本，替换原有的 HTTP Method。
    - [x] 验证：启动程序，访问目录和下载文件，观察 TUI 日志输出。

4.  **美化与对齐**
    - [x] 调整日志行格式，确保动作标签对齐美观。
    - [x] 验证：UI 视觉效果符合预期。
