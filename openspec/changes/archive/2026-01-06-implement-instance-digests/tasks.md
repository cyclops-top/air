# Tasks: Implement Instance Digests

1.  **准备依赖**
    - [x] 在 `Cargo.toml` 中添加 `sha2`, `base64`, `dashmap`。
    - [x] 验证：`make build` 成功。

2.  **实现哈希计算工具**
    - [x] 在 `src/fs_utils.rs` 中实现 `calculate_sha256(path: &Path) -> Result<String>`。
    - [x] 使用流式读取确保低内存占用。
    - [x] 验证：单元测试对比小文件的已知哈希值。

3.  **扩展应用状态与缓存**
    - [x] 在 `src/handlers.rs` 中定义 `DigestEntry`。
    - [x] 为 `AppState` 增加 `digest_cache` 字段。
    - [x] 验证：编译通过。

4.  **集成 Digest 响应头**
    - [x] 修改 `src/handlers.rs::handle_request`。
    - [x] 实现缓存检查、计算、更新逻辑。
    - [x] 确保在 200 和 206 响应中均注入 `Digest` 头部。
    - [x] 验证：使用 `curl -I` 观察响应头中是否存在 `Digest: SHA-256=...`。

5.  **性能与正确性验证**
    - [x] 验证文件修改后（通过 `touch` 修改 mtime），`Digest` 头部能自动更新。
    - [x] 验证 Range 请求返回的是完整文件哈希。
