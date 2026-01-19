use async_trait::async_trait;
use crate::domain::models::{DiscoveryMsg, FileEntry};
use std::path::Path;
use bytes::Bytes;

#[async_trait]
pub trait DiscoveryProvider: Send + Sync {
    /// 注册当前服务，返回完整的服务名称（fullname）
    fn register(&self, msg: &DiscoveryMsg) -> anyhow::Result<String>;
    
    /// 注销服务
    fn unregister(&self, fullname: &str) -> anyhow::Result<()>;

    /// 持续监听局域网服务，通过 channel 发送发现的消息
    async fn listen(&self, tx: tokio::sync::mpsc::Sender<DiscoveryMsg>, shutdown_rx: tokio::sync::oneshot::Receiver<()>) -> anyhow::Result<()>;
}

#[async_trait]
pub trait FileRepository: Send + Sync {
    /// 遍历目录获取文件列表
    async fn list_directory(&self, abs_path: &Path, request_path: &str) -> anyhow::Result<Vec<FileEntry>>;
    
    /// 获取文件的字节内容（支持 Range）
    async fn get_file_content(&self, abs_path: &Path, range: Option<std::ops::Range<usize>>) -> anyhow::Result<Bytes>;
    
    /// 获取或计算文件摘要
    async fn get_file_digest(&self, abs_path: &Path) -> anyhow::Result<String>;
}

pub trait UiRenderer: Send + Sync {
    /// 渲染 HTML 页面
    fn render_html(&self, listing: &crate::domain::models::DirectoryListing) -> String;
}