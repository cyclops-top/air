# Tasks: Implement Cross Compilation

1.  **更新 Makefile 定义**
    - [x] 定义目标架构变量 (`TARGET_LINUX_GNU`, `TARGET_MACOS_INTEL`, etc.)。
    - [x] 定义输出目录变量 (`DIST_DIR`).
    - [x] 验证：运行 `make -n build-linux` 能看到正确的 cargo 命令。

2.  **实现各平台构建目标**
    - [x] 实现 `build-linux` 目标：编译并复制到 `dist/air-linux-amd64`。
    - [x] 实现 `build-macos` 目标：编译 Intel 和 ARM 版本并复制。
    - [x] 实现 `build-windows` 目标：编译并复制到 `dist/air-windows-amd64.exe`。
    - [x] 实现 `build-all` 聚合目标。
    - [x] 验证：在具备环境的机器上运行构建（或者至少验证命令生成正确）。

3.  **添加辅助目标**
    - [x] 实现 `install-targets` 目标：调用 `rustup target add` 安装所需 targets。
    - [x] 实现 `clean-dist` 目标：清理 `dist` 目录。
    - [x] 验证：运行 `make install-targets`。
