# Tasks: Decode Log Paths

1.  **实现路径解码**
    - [ ] 修改 `src/logger.rs` 中的 `log_request` 函数。
    - [ ] 对 `path` 变量进行 `percent_decode_str` 处理并转换为 UTF-8 字符串。
    - [ ] 验证：访问包含空格的文件（如 `test file.txt`），TUI 日志显示为正常空格而非 `%20`。

2.  **全面检查**
    - [ ] 运行 `make build` 和 `make test`。
    - [ ] 验证：功能正常且无编译警告。
