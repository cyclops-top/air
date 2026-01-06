# Design: Favicon Handling

## 1. 路由拦截

在 `src/server.rs` 的 `Router` 中，在 `fallback` 之前添加一个显式的路由：
`.route("/favicon.ico", get(handlers::favicon))`

## 2. Handler 实现

在 `src/handlers.rs` 中实现 `favicon` 函数：
- 获取当前工作目录下的 `docs/favicon.svg` 路径。
- 使用 `tower_http::services::ServeFile` 直接服务该文件。
- 注意：由于这个文件不在用户指定的 `root_path` 沙箱内，我们不调用 `fs_utils::sanitize_path`，而是直接引用程序自身的静态资源。

## 3. HTML 头部更新

在 `src/view.rs` 的 `render_html` 函数中添加：
`<link rel='icon' type='image/svg+xml' href='/favicon.ico'>`
虽然请求的是 `.ico` 路径，但通过指定 `type='image/svg+xml'`，现代浏览器可以正确处理返回的 SVG 内容。
