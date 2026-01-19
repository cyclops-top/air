use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::sync::atomic::AtomicU64;
use std::sync::Mutex;
use std::collections::VecDeque;
use std::time::{Duration, Instant, SystemTime};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscoveryMsg {
    pub id: String,
    pub name: String,
    pub ip: IpAddr,
    pub port: u16,
    pub scheme: String,
    #[serde(rename = "isOnline")]
    pub is_online: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub name: String,
    #[serde(rename = "isDir")]
    pub is_dir: bool,
    pub size: u64,
    #[serde(rename = "modTime")]
    pub mod_time: String,
    #[serde(rename = "mediaType")]
    pub media_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirectoryListing {
    #[serde(rename = "currentPath")]
    pub current_path: String,
    pub items: Vec<FileEntry>,
    #[serde(rename = "lanIp")]
    pub lan_ip: String,
    pub port: u16,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum LogAction {
    OpenDir,
    DownloadFile,
    Favicon,
}

pub struct LogEntry {
    pub time: String,
    #[allow(dead_code)]
    pub ip: String,
    pub action: LogAction,
    pub duration: Duration,
    pub path: String,
    pub is_success: bool,
    pub range: Option<String>,
}

pub struct Stats {
    pub total_files: AtomicU64,
    pub total_bytes: AtomicU64,
    pub logs: Mutex<VecDeque<LogEntry>>,
    pub start_time: Instant,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            total_files: AtomicU64::new(0),
            total_bytes: AtomicU64::new(0),
            logs: Mutex::new(VecDeque::new()),
            start_time: Instant::now(),
        }
    }
}

pub struct DigestEntry {
    pub hash: String,
    pub mtime: SystemTime,
    pub size: u64,
}

pub struct AppState {
    pub root_path: PathBuf,
    pub stats: std::sync::Arc<Stats>,
    #[allow(dead_code)]
    pub enable_https: bool,
    pub lan_ip: String,
    pub port: u16,
}