# Tasks: Add Quit Logic and Download Statistics

1.  **更新 AppState 与统计逻辑**
    - [x] 在 `src/handlers.rs` 的 `AppState` 中添加原子计数器。
    - [x] 在 `src/handlers.rs` 的 `handle_request` 中实现文件下载统计累加。
    - [x] 验证：通过日志确认每次文件请求都会增加计数。

2.  **实现优雅关闭与 'q' 键监听**
    - [x] 在 `src/server.rs` 中实现 `shutdown_signal` 函数，支持 `ctrl-c` 和 `q` 输入。
    - [x] 更新 `axum::serve` 调用以支持优雅关闭。
    - [x] 验证：按下 'q' 键（后跟回车）程序应正常结束。

3.  **展示运行统计**
    - [x] 修改 `src/main.rs` 以在服务停止后打印统计摘要。
    - [x] 更新启动横幅，告知用户支持 'q' 键退出。
    - [x] 验证：程序退出后在终端显示正确的文件数和总字节量。

4.  **代码清理与优化**
    - [x] 确保统计逻辑不影响目录浏览。
    - [x] 验证：全流程手动测试，确保统计准确。
