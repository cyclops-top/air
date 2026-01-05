# Proposal: 添加跨平台编译支持

## 摘要
本提案旨在更新项目的构建系统（`Makefile`），以支持为多种操作系统和架构编译 `Air` 二进制文件。

## 目标
1.  **多平台支持**：支持编译 Linux (x86_64), macOS (x86_64, aarch64), Windows (x86_64) 的二进制文件。
2.  **Makefile 集成**：在 `Makefile` 中添加新的目标命令，以便一键生成所有平台的构建产物。
3.  **产物管理**：将不同平台的二进制文件输出到统一的 `bin/` 目录中，并按平台命名。

## 范围
*   **Makefile**: 新增 `build-all`, `build-linux`, `build-macos`, `build-windows` 等目标。
*   **Rust Toolchain**: 依赖 `rustup` 添加相应的 target 支持。
*   **Cross Compilation Strategy**: 优先使用标准 `cargo build --target`。
    *   *注意：从 macOS 交叉编译 Linux/Windows 可能需要额外的链接器支持。本提案首先定义构建目标和逻辑，用户需确保环境满足（如安装 `musl-cross` 或 `mingw`）。*

## 关键变更
*   修改 `air/Makefile`。
