use async_trait::async_trait;
use crate::domain::models::FileEntry;
use std::path::Path;
use tokio::io::AsyncRead;
use std::pin::Pin;

pub type FileStream = Pin<Box<dyn AsyncRead + Send + Sync>>;

#[async_trait]
pub trait FileRepository: Send + Sync {
    /// 遍历目录获取文件列表
    async fn list_directory(&self, abs_path: &Path, request_path: &str) -> anyhow::Result<Vec<FileEntry>>;

    /// 获取文件内容流
    /// 返回一个异步读取器，允许流式传输
    async fn get_file_stream(&self, abs_path: &Path, range: Option<std::ops::Range<u64>>) -> anyhow::Result<(FileStream, u64)>;

    /// 获取文件摘要 (SHA256)
    /// 对于大文件，建议使用 mtime + size 生成弱哈希以避免阻塞
    async fn get_file_digest(&self, abs_path: &Path) -> anyhow::Result<String>;
}

pub trait UiRenderer: Send + Sync {
    fn render_html(&self, listing: &crate::domain::models::DirectoryListing) -> String;
}

pub trait DiscoveryProvider: Send + Sync {
    fn register_service(&self, msg: &crate::domain::models::DiscoveryMsg) -> anyhow::Result<String>;
    fn unregister_service(&self, fullname: &str) -> anyhow::Result<()>;
    fn start_discovery(&self, tx: tokio::sync::mpsc::Sender<crate::domain::models::DiscoveryMsg>) -> anyhow::Result<()>;
}
