# Tasks: Implement Clickable Breadcrumbs

1.  **实现面包屑生成逻辑**
    - [x] 在 `view.rs` 中实现 `render_breadcrumbs` 函数。
    - [x] 函数应处理根路径 `/`、深层路径 `/a/b/c` 以及特殊字符编码。
    - [x] 验证：单元测试验证生成的 HTML 字符串正确。

2.  **集成到 Web UI**
    - [x] 修改 `render_html` 调用 `render_breadcrumbs`。
    - [x] 移除旧的 "Current: " 文本展示。
    - [x] 验证：浏览器访问深度目录，点击面包屑可跳转。

3.  **样式美化**
    - [x] 更新 `render_html` 中的 `<style>` 块。
    - [x] 添加分隔符样式和面包屑链接样式。
    - [x] 验证：UI 视觉效果符合预期。
