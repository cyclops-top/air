# Tasks: Bundle Static Assets

1.  **修改 src/handlers.rs 实现嵌入**
    - [x] 定义 `FAVICON_SVG` 常量并使用 `include_bytes!`。
    - [x] 重写 `favicon` handler 以返回内存中的字节。
    - [x] 验证：`make build` 成功。

2.  **验证图标服务**
    - [x] 启动服务并在浏览器访问 `/favicon.ico`。
    - [x] 验证：在删除 `docs/favicon.svg` 的情况下（或在不同目录下运行），图标依然能正常显示。

3.  **清理代码**
    - [x] 移除 `favicon` handler 中不再需要的 `std::path::PathBuf` 和 `tower_http::services::ServeFile` 调用。
    - [x] 验证：`make lint` 通过。
