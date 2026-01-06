# Tasks: Implement ETag Support

1.  **实现 ETag 校验逻辑**
    - [x] 修改 `src/handlers.rs` 中的 `handle_request`。
    - [x] 在获取到 `hash` 之后，提取请求头的 `If-None-Match` 并进行比对。
    - [x] 如果匹配，返回 `StatusCode::NOT_MODIFIED`。
    - [x] 验证：使用 `curl -H 'If-None-Match: "..."'` 验证返回 304。

2.  **注入 ETag 响应头**
    - [x] 在文件服务成功的响应中，注入 `ETag` 头部。
    - [x] 验证：观察 `curl -I` 输出，确认 `ETag` 已被引号包裹。

3.  **集成与回归测试**
    - [x] 确保 `Range` 请求在 ETag 存在时依然正常工作。
    - [x] 确保 `Last-Modified` 依然正确输出。
    - [x] 验证：全流程自动化测试通过。

4.  **性能观察**
    - [x] 验证在 304 响应下，TUI 的流量统计（Volume）不应增加（或仅增加头部大小）。
