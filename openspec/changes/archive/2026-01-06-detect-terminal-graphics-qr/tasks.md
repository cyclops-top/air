# Tasks: Implement Graphics-based QR Code

1.  **依赖与环境准备**
    - [x] 在 `Cargo.toml` 中添加 `ratatui-image`。
    - [x] 验证：`make build` 成功。

2.  **重构 DashboardState**
    - [x] 在 `src/dashboard.rs` 的 `DashboardState` 中添加 `picker` 和 `image_state`。
    - [x] 在 `src/main.rs` 中初始化 `Picker` 并传入状态。
    - [x] 验证：在支持 Sixel 的终端启动，日志或调试信息显示检测到协议。

3.  **实现图像二维码生成**
    - [x] 在 `src/dashboard.rs` 中实现将 URL 转换为 `DynamicImage` 的函数。
    - [x] 验证：生成逻辑不报错。

4.  **实现条件渲染逻辑**
    - [x] 根据 `picker` 是否可用，动态切换 Header 布局（单列或双列）。
    - [x] 使用 `ratatui-image` 的 Widget 渲染生成的二维码。
    - [x] 验证：
        - 支持图形的终端（如 WezTerm, iTerm2）显示高清二维码。
        - 不支持图形的终端（如原始 CMD, 旧版 VSCode 终端）不显示二维码，且布局正常。

5.  **清理与收尾**
    - [x] 移除旧的 Unicode 二维码渲染代码。
    - [x] 验证：`make test` 通过。
