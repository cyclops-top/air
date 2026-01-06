# Design: HTTP Instance Digests with In-Memory Cache

## 1. 数据结构

### 1.1 缓存模型
使用 `dashmap` 实现高性能并发访问。
```rust
pub struct DigestEntry {
    pub hash: String, // Base64 encoded SHA-256
    pub mtime: std::time::SystemTime,
    pub size: u64,
}

pub struct AppState {
    // ... 现有字段
    pub digest_cache: dashmap::DashMap<PathBuf, DigestEntry>,
}
```

## 2. 哈希计算逻辑

在 `src/fs_utils.rs` 中增加 `calculate_digest`：
- 使用 `tokio::fs::File` 流式读取。
- 使用 `sha2::Sha256` 逐步更新。
- 计算完成后转换为 Base64 字符串（符合 RFC 3230 标准）。

## 3. 请求处理流程 (src/handlers.rs)

1.  定位到目标文件 `abs_path`。
2.  获取文件元数据 `(mtime, size)`。
3.  检查 `digest_cache`:
    - 如果存在且 `mtime` & `size` 匹配：获取 `hash`。
    - 否则：执行哈希计算，更新缓存。
4.  构造响应头：`Digest: SHA-256=<hash>`。
5.  注入 `Want-Digest` 支持 (可选但推荐): 如果请求包含 `Want-Digest`，优先满足。

## 4. 特殊场景处理

### 4.1 Range 请求
根据 RFC 3230，`Digest` 反映的是“实例”（整个文件）的摘要，即便响应的是 206 Partial Content，也必须返回整个文件的 SHA-256。

### 4.2 大文件
对于首次请求的大文件，计算过程可能耗时。
*方案*：同步计算以确保响应头准确（简单且符合规范）。
