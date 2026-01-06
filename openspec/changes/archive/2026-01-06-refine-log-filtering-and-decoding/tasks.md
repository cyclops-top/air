# Tasks: Refine Log Filtering and Decoding

1.  **修改日志中间件**
    - [x] 更新 `src/logger.rs` 中的 `log_request` 函数。
    - [x] 增加对请求方法是否为 `GET` 的检查。
    - [x] 实现对路径字符串的 URL 解码。
    - [x] 验证：使用 `curl -I` (HEAD 请求) 访问，TUI 不应显示新日志；使用 `curl` (GET 请求) 访问带空格的文件，显示正常空格。

2.  **验证与清理**
    - [x] 确保 `make test` 依然通过。
    - [x] 验证 TUI 界面中的路径显示符合预期。
