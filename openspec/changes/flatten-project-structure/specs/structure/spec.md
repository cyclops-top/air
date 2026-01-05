# Structure Requirements

## MODIFIED Requirements

### Requirement: 项目目录布局
项目的源代码和构建文件 MUST 位于仓库的根目录。

#### Scenario: 根目录结构
完成结构调整后，根目录必须包含：
- `Cargo.toml` (Rust 项目配置)
- `src/` (源代码目录)
- `Makefile` (构建自动化)
- `docs/` (文档)
- `openspec/` (OpenSpec 规范)

#### Scenario: 移除二级目录
仓库中 MUST 不再包含名为 `air/` 的用于存放 Rust 项目代码的二级目录。
