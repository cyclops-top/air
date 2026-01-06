# Design: Flatten Project Structure

## 1. 移动计划

将执行以下移动操作：
- `air/Cargo.toml` -> `./Cargo.toml`
- `air/Cargo.lock` -> `./Cargo.lock`
- `air/Makefile` -> `./Makefile`
- `air/src/` -> `./src/`
- `air/.gitignore` -> `./.gitignore`
- `air/.git/` -> `./.git/`
- `air/dist/` -> `./dist/` (如果存在)
- `air/target/` -> `./target/` (如果存在)

## 2. 影响评估

### 2.1 Makefile
`air/Makefile` 中的路径通常是相对于 `Makefile` 所在位置的。
例如：
- `cargo build` 会在当前目录下寻找 `Cargo.toml`。
- `cp target/...` 也是相对于当前目录。
移动到根目录后，这些相对路径逻辑保持不变，因此 `Makefile` 内容预计无需大幅修改。

### 2.2 Git
移动 `.git` 文件夹后，git 仓库将管理整个根目录及其子目录（包括 `docs/` 和 `openspec/`）。这符合通常的项目管理习惯。

### 2.3 路径引用
在之前的 OpenSpec 任务或文档中提到的 `air/` 路径前缀在实现时将不再需要。
