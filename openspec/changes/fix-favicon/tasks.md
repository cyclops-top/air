# Tasks: Fix Favicon Error

1.  **实现 Favicon Handler**
    - [x] 在 `src/handlers.rs` 中添加 `favicon` 异步函数。
    - [x] 函数应直接读取 `docs/favicon.svg` 并返回。
    - [x] 验证：单元测试或手动 curl 验证 `/favicon.ico` 返回 SVG 内容。

2.  **更新服务器路由**
    - [x] 在 `src/server.rs` 中注册 `/favicon.ico` 路由。
    - [x] 确保其优先级高于 `fallback`。
    - [x] 验证：启动服务后，访问 `http://localhost:port/favicon.ico` 不再报错。

3.  **更新 Web UI 模板**
    - [x] 在 `src/view.rs` 中为 HTML `<head>` 添加 favicon 链接。
    - [x] 验证：浏览器标签页显示正确的图标。
