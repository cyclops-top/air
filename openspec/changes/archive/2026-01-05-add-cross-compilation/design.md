# Design: Cross Compilation in Makefile

## 1. Target Platforms

我们将支持以下核心目标平台：
*   `x86_64-apple-darwin` (macOS Intel)
*   `aarch64-apple-darwin` (macOS Apple Silicon)
*   `x86_64-unknown-linux-gnu` (Linux x86_64) - *注：推荐使用 musl (`x86_64-unknown-linux-musl`) 以获得静态链接的二进制，但 gnu 更通用。此处先支持 gnu。*
*   `x86_64-pc-windows-gnu` (Windows x86_64) - *使用 gnu 工具链通常比 msvc 在交叉编译时更容易配置。*

## 2. Makefile 结构设计

```makefile
# Define targets
TARGET_LINUX_GNU = x86_64-unknown-linux-gnu
TARGET_MACOS_INTEL = x86_64-apple-darwin
TARGET_MACOS_ARM = aarch64-apple-darwin
TARGET_WIN_GNU = x86_64-pc-windows-gnu

# Output directory
DIST_DIR = dist

.PHONY: build-all build-linux build-macos build-windows

# Build All
build-all: build-linux build-macos build-windows

# Linux
build-linux:
	cargo build --release --target $(TARGET_LINUX_GNU)
	mkdir -p $(DIST_DIR)
	cp target/$(TARGET_LINUX_GNU)/release/air $(DIST_DIR)/air-linux-amd64

# MacOS
build-macos:
	cargo build --release --target $(TARGET_MACOS_INTEL)
	cargo build --release --target $(TARGET_MACOS_ARM)
	mkdir -p $(DIST_DIR)
	cp target/$(TARGET_MACOS_INTEL)/release/air $(DIST_DIR)/air-darwin-amd64
	cp target/$(TARGET_MACOS_ARM)/release/air $(DIST_DIR)/air-darwin-arm64

# Windows
build-windows:
	cargo build --release --target $(TARGET_WIN_GNU)
	mkdir -p $(DIST_DIR)
	cp target/$(TARGET_WIN_GNU)/release/air.exe $(DIST_DIR)/air-windows-amd64.exe
```

## 3. 前置检查
可以添加一个 `check-tools` 目标，检查 `rustup target list --installed` 是否包含所需 target，如果没有则提示用户安装：
`rustup target add x86_64-unknown-linux-gnu ...`
