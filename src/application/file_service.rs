use crate::domain::models::{DirectoryListing, AppState};
use crate::domain::traits::{FileRepository, FileStream};
use std::sync::Arc;
use std::path::Path;
use std::sync::atomic::Ordering;

pub struct FileService {
    repo: Arc<dyn FileRepository>,
    state: Arc<AppState>,
}

impl FileService {
    pub fn new(repo: Arc<dyn FileRepository>, state: Arc<AppState>) -> Self {
        Self { repo, state }
    }

    pub async fn get_listing(&self, request_path: &str, abs_path: &Path) -> anyhow::Result<DirectoryListing> {
        let items = self.repo.list_directory(abs_path, request_path).await?;
        Ok(DirectoryListing {
            current_path: request_path.to_string(),
            items,
            lan_ip: self.state.lan_ip.clone(),
            port: self.state.port,
        })
    }

    pub async fn get_file_content(&self, abs_path: &Path, range: Option<std::ops::Range<u64>>) -> anyhow::Result<(FileStream, u64, String)> {
        // 1. 获取摘要（用于 ETag）
        let hash = self.repo.get_file_digest(abs_path).await?;
        
        // 2. 更新统计信息 (仅在非 Range 请求或 Range 起始为 0 时统计一次下载)
        if range.as_ref().map(|r| r.start == 0).unwrap_or(true) {
            let metadata = std::fs::metadata(abs_path)?;
            self.state.stats.total_files.fetch_add(1, Ordering::Relaxed);
            self.state.stats.total_bytes.fetch_add(metadata.len(), Ordering::Relaxed);
        }

        // 3. 获取流
        let (stream, length) = self.repo.get_file_stream(abs_path, range).await?;
        Ok((stream, length, hash))
    }
}
