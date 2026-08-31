use async_trait::async_trait;
use crate::domain::models::{FileEntry, DigestEntry};
use crate::domain::traits::{FileRepository, FileStream};
use crate::fs_utils;
use std::path::{Path, PathBuf};
use dashmap::DashMap;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

pub struct LocalFileRepository {
    pub digest_cache: DashMap<PathBuf, DigestEntry>,
}

impl LocalFileRepository {
    pub fn new() -> Self {
        Self {
            digest_cache: DashMap::new(),
        }
    }
}

#[async_trait]
impl FileRepository for LocalFileRepository {
    async fn list_directory(&self, abs_path: &Path, _request_path: &str) -> anyhow::Result<Vec<FileEntry>> {
        let read_dir = std::fs::read_dir(abs_path)?;
        let mut items = Vec::new();

        for entry in read_dir.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') { continue; }

            let meta = entry.metadata().ok();
            let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
            let is_dir = meta.as_ref().map(|m| m.is_dir()).unwrap_or(false);
            let mod_time = if let Some(m) = meta {
                m.modified().map(|t| {
                    let dt: chrono::DateTime<chrono::Local> = t.into();
                    dt.to_rfc3339()
                }).unwrap_or_default()
            } else { "".to_string() };

            let media_type = if is_dir {
                "directory".to_string()
            } else {
                mime_guess::from_path(&name).first_or_octet_stream().to_string()
            };

            items.push(FileEntry { name, is_dir, size, mod_time, media_type });
        }

        items.sort_by(|a, b| {
            if a.is_dir && !b.is_dir { std::cmp::Ordering::Less }
            else if !a.is_dir && b.is_dir { std::cmp::Ordering::Greater }
            else { a.name.to_lowercase().cmp(&b.name.to_lowercase()) }
        });

        Ok(items)
    }

    async fn get_file_stream(&self, abs_path: &Path, range: Option<std::ops::Range<u64>>) -> anyhow::Result<(FileStream, u64)> {
        let mut file = File::open(abs_path).await?;
        let metadata = file.metadata().await?;
        let total_size = metadata.len();

        if let Some(r) = range {
            file.seek(std::io::SeekFrom::Start(r.start)).await?;
            // 限制读取长度
            let length = r.end - r.start;
            let take = file.take(length);
            Ok((Box::pin(take), length))
        } else {
            Ok((Box::pin(file), total_size))
        }
    }

    async fn get_file_digest(&self, abs_path: &Path) -> anyhow::Result<String> {
        let metadata = std::fs::metadata(abs_path)?;
        let mtime = metadata.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        let size = metadata.len();

        // 策略优化：大文件 (>10MB) 使用弱哈希 (mtime + size)，避免全量读取造成阻塞
        if size > 10 * 1024 * 1024 {
            let weak_key = format!("{:?}-{}", mtime, size);
            // 使用简单的 MD5 或直接字符串作为 ETag
            // 这里为了性能，直接返回 Base64 编码的元数据标识
            use base64::{Engine as _, engine::general_purpose};
            return Ok(general_purpose::URL_SAFE_NO_PAD.encode(weak_key));
        }

        if let Some(entry) = self.digest_cache.get(abs_path) {
            if entry.mtime == mtime && entry.size == size {
                return Ok(entry.hash.clone());
            }
        }

        let hash = fs_utils::calculate_sha256(abs_path).await?;
        self.digest_cache.insert(abs_path.to_path_buf(), DigestEntry {
            hash: hash.clone(),
            mtime,
            size,
        });
        Ok(hash)
    }
}

// 移除不再需要的 MmapCache 结构体
pub struct MmapCache;
impl MmapCache { pub fn new() -> Self { Self } }