# Design: Terminal Graphics Detection and QR Rendering

## 1. 协议探测 (Detection)

使用 `ratatui-image` 的 `Picker` 模块：
```rust
use ratatui_image::picker::Picker;

// 在程序启动或 Dashboard 初始化时
let mut picker = Picker::from_termios().ok();
```
`Picker` 会自动尝试各种查询方式（DA1, DA2, 控制序列等）来识别 Sixel、Kitty 图形、iTerm2 图形等支持情况。

## 2. 图像生成 (Generation)

利用 `qrcode` crate 的 `render::<image::Luma<u8>>()` 模式生成 `ImageBuffer`：
```rust
let code = QrCode::new(url.as_bytes())?;
let image = code.render::<Luma<u8>>().build();
// 转换为 DynamicImage 以供 ratatui-image 使用
let dyn_img = DynamicImage::ImageLuma8(image);
```

## 3. 动态 UI 布局 (Conditional Layout)

在 `src/dashboard.rs` 的 `render` 函数中：
1. 检查 `picker` 是否存在。
2. 如果存在：
    - 将 Header 分割为 `[36, Min(0)]`。
    - 在左侧使用 `ratatui_image::StatefulImage` 渲染二维码。
3. 如果不存在：
    - 不进行水平分割，Header 区域全部用于显示系统信息。

## 4. 状态持久化

将 `Picker` 实例保存在 `DashboardState` 中，避免每一帧都重新探测。
注意：`ratatui-image` 需要在 `StatefulWidget` 模式下使用 `ImageState` 来追踪图像在终端中的位置。
