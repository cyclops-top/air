# build Specification

## Purpose
TBD - created by archiving change add-cross-compilation. Update Purpose after archive.
## Requirements
### Requirement: 跨平台构建支持
Build 系统 MUST 支持通过简单的 Makefile 命令生成 Linux, macOS 和 Windows 的发布版二进制文件。

#### Scenario: 构建 Linux 版本
用户运行 `make build-linux`。
系统编译 `x86_64-unknown-linux-gnu` target。
生成的二进制文件位于 `dist/air-linux-amd64`。

#### Scenario: 构建 macOS 版本
用户运行 `make build-macos`。
系统编译 `x86_64-apple-darwin` 和 `aarch64-apple-darwin` targets。
生成的二进制文件分别位于 `dist/air-darwin-amd64` 和 `dist/air-darwin-arm64`。

#### Scenario: 构建 Windows 版本
用户运行 `make build-windows`。
系统编译 `x86_64-pc-windows-gnu` target。
生成的二进制文件位于 `dist/air-windows-amd64.exe`。

#### Scenario: 一键构建所有
用户运行 `make build-all`。
系统依次构建上述所有平台。

### Requirement: 工具链安装辅助
Build 系统 MUST 提供辅助命令来安装缺失的 Rust targets。

#### Scenario: 安装 Targets
用户运行 `make install-targets`。
系统调用 `rustup` 安装 Linux, macOS (Intel/ARM), Windows 对应的 targets。

