# Tasks: Implement TUI QR Code

1.  **添加依赖**
    - [x] 在 `Cargo.toml` 中添加 `qrcode` 依赖。
    - [x] 验证：`make build` 成功。

2.  **实现二维码生成逻辑**
    - [x] 在 `src/dashboard.rs` 中编写辅助逻辑，根据协议和 IP 生成二维码 Unicode 字符串。
    - [x] 验证：单元测试验证生成的字符串格式正确。

3.  **重构 TUI 布局**
    - [x] 修改 `src/dashboard.rs` 的 `render` 函数，对 Header 块进行水平分割。
    - [x] 在右侧区域渲染二维码。
    - [x] 验证：运行 `make run` 观察全屏模式下二维码的显示效果。

4.  **适配与优化**
    - [x] 调整 Header 块的高度或比例，确保二维码显示完整（不被截断）。
    - [x] 验证：调整终端窗口大小时，二维码依然可读。
