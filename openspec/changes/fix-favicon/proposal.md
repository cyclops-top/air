# Proposal: 修复 favicon.ico 403 错误并支持 SVG 图标

## 摘要
目前浏览器请求 `/favicon.ico` 时会触发 403 Forbidden 错误，因为该路径不在共享目录沙箱内。本提案建议在服务器路由中添加对 `/favicon.ico` 的特殊处理，并映射到项目中的 `docs/favicon.svg` 文件。

## 目标
1.  **消除 403 错误**：通过在路由层拦截 `/favicon.ico` 请求，避免其进入文件系统的安全检查逻辑。
2.  **自定义图标**：使用项目自带的 `docs/favicon.svg` 作为服务端的图标。
3.  **兼容性**：确保浏览器能够正确识别并显示 SVG 格式的图标。

## 范围
*   **src/server.rs**: 添加 `/favicon.ico` 路由。
*   **src/handlers.rs**: 实现处理 `/favicon.ico` 的 handler，读取并返回 `docs/favicon.svg` 内容。
*   **src/view.rs**: (可选) 在 HTML 头部显式指定 favicon 路径。
