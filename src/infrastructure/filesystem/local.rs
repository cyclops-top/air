use async_trait::async_trait;
use crate::domain::models::{FileEntry, DigestEntry};
use crate::domain::traits::FileRepository;
use crate::fs_utils;
use bytes::Bytes;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Weak};
use memmap2::Mmap;
use dashmap::DashMap;

pub struct LocalFileRepository {
    pub mmap_cache: Arc<MmapCache>,
    pub digest_cache: DashMap<PathBuf, DigestEntry>,
}

impl LocalFileRepository {
    pub fn new(mmap_cache: Arc<MmapCache>) -> Self {
        Self {
            mmap_cache,
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

            items.push(FileEntry { name, is_dir, size, mod_time });
        }

        items.sort_by(|a, b| {
            if a.is_dir && !b.is_dir { std::cmp::Ordering::Less }
            else if !a.is_dir && b.is_dir { std::cmp::Ordering::Greater }
            else { a.name.to_lowercase().cmp(&b.name.to_lowercase()) }
        });

        Ok(items)
    }

    async fn get_file_content(&self, abs_path: &Path, range: Option<std::ops::Range<usize>>) -> anyhow::Result<Bytes> {
        let mapped = self.mmap_cache.get_or_create(abs_path)?;
        if let Some(r) = range {
            Ok(Bytes::copy_from_slice(&mapped.mmap[r]))
        } else {
            Ok(Bytes::copy_from_slice(&mapped.mmap[..]))
        }
    }

    async fn get_file_digest(&self, abs_path: &Path) -> anyhow::Result<String> {
        let metadata = std::fs::metadata(abs_path)?;
        let mtime = metadata.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH);
        let size = metadata.len();

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

pub struct MappedFile {
    pub mmap: Mmap,
    pub path: PathBuf,
    pub cache: Arc<MmapCache>,
}

impl Drop for MappedFile { fn drop(&mut self) { self.cache.remove(&self.path); } }

pub struct MmapCache {
    pub mappings: DashMap<PathBuf, Weak<MappedFile>>,
}

impl MmapCache {
    pub fn new() -> Self { Self { mappings: DashMap::new() } }
    pub fn get_or_create(self: &Arc<Self>, path: &Path) -> std::io::Result<Arc<MappedFile>> {
        if let Some(weak) = self.mappings.get(path) { if let Some(arc) = weak.upgrade() { return Ok(arc); } }
        let file = std::fs::File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        let mapped_file = Arc::new(MappedFile { mmap, path: path.to_path_buf(), cache: self.clone() });
        self.mappings.insert(path.to_path_buf(), Arc::downgrade(&mapped_file));
        Ok(mapped_file)
    }
    fn remove(&self, path: &Path) { if let Some(weak) = self.mappings.get(path) { if weak.upgrade().is_none() { self.mappings.remove(path); } } }
}
