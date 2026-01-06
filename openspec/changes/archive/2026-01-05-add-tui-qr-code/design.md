# Design: TUI QR Code Dashboard Integration

## 1. 布局重构

在 `src/dashboard.rs` 中，现有的顶部 Header 区域（高度为 10）将进行水平分割：
- **左侧 (70%-80%)**: 保持现有的 `System Status` 信息（路径、Local/Network 地址、统计数据等）。
- **右侧 (20%-30%)**: 渲染 `Scan to Connect` 二维码。

## 2. 二维码生成

使用 `qrcode` crate：
```rust
let url = format!("{}://{}:{}", protocol, ui_state.lan_ip, ui_state.port);
let code = QrCode::new(url.as_bytes()).unwrap();
let string = code.render::<unicode::Dense1x2>().build();
```
使用 `unicode::Dense1x2` 渲染模式，每个终端字符可以代表两个二维码像素，从而缩小垂直高度，使其更适合在 TUI 中展示。

## 3. 渲染实现

- **Widget**: 使用 `Paragraph` 或自定义逻辑渲染生成的 Unicode 字符串。
- **Block**: 为二维码区域添加边框，标题为 " Scan me "。

## 4. 适配性

如果终端窗口过窄，二维码区域可能会被压缩或隐藏，以确保基本的文本信息可见。
