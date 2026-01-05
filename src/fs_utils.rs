use std::path::{Path, PathBuf};
use anyhow::{anyhow, Result};

/// Sanitizes the request path to ensure it is safely within the root directory.
/// 
/// 1. Joins root_dir with request_path (decoded).
/// 2. Canonicalizes the path (resolves symlinks, .., .).
/// 3. Checks if the result starts with root_dir.
/// 
/// Returns the absolute path if safe, or an error.
pub fn sanitize_path(root_dir: &Path, request_path: &str) -> Result<PathBuf> {
    // 1. Decode URL path (basic handling, axum might do this but we double check)
    // Actually, the path passed here should be the raw path segment.
    // For now assume request_path is a relative path string like "folder/file.txt"
    // Remove leading slash if present to ensure join works as relative
    let relative_path = request_path.trim_start_matches('/');
    
    // Prevent absolute paths in request_path from resetting the join
    if Path::new(relative_path).is_absolute() {
         return Err(anyhow!("Invalid path: absolute path not allowed"));
    }

    let candidate = root_dir.join(relative_path);

    // 2. Canonicalize
    // Note: canonicalize requires the path to exist. 
    // If it doesn't exist, we can't serve it anyway (404). 
    // But for security check, if it exists, we MUST check boundaries.
    match candidate.canonicalize() {
        Ok(abs_path) => {
            // 3. Prefix check
            if abs_path.starts_with(root_dir) {
                Ok(abs_path)
            } else {
                Err(anyhow!("Access denied: Path traversal attempt"))
            }
        }
        Err(e) => {
            // If file not found, it is safe in terms of access (can't read what doesn't exist)
            // But we should return the error so caller can 404.
            Err(anyhow::Error::new(e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::os::unix::fs::symlink;
    use tempfile::TempDir;

    #[test]
    fn test_normal_access() {
        let temp = TempDir::new().unwrap();
        let root = temp.path().canonicalize().unwrap();
        
        let file_path = root.join("test.txt");
        File::create(&file_path).unwrap();

        let result = sanitize_path(&root, "test.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), file_path);
    }

    #[test]
    fn test_traversal_attempt() {
        let temp = TempDir::new().unwrap();
        let root = temp.path().canonicalize().unwrap();
        
        // request ../../etc/passwd (assuming we are deep enough or just trying to go up)
        // Since root is temp, .. goes to /tmp (usually). 
        // Let's try to access parent of root.
        let parent = root.parent().unwrap();
        let result = sanitize_path(&root, "../");
        
        // Should fail because parent is outside root
        assert!(result.is_err());
    }

    #[test]
    fn test_symlink_escape() {
        let temp = TempDir::new().unwrap();
        let root = temp.path().canonicalize().unwrap();
        
        // Create a secret file outside root
        let secret_dir = TempDir::new().unwrap();
        let secret_file = secret_dir.path().join("secret.txt");
        File::create(&secret_file).unwrap();

        // Create a symlink inside root pointing to secret file
        let link_path = root.join("link_to_secret");
        symlink(&secret_file, &link_path).unwrap();

        // Try to access via symlink
        let result = sanitize_path(&root, "link_to_secret");
        
        // Should fail because it resolves to outside root
        assert!(result.is_err());
    }

    #[test]
    fn test_nested_file() {
        let temp = TempDir::new().unwrap();
        let root = temp.path().canonicalize().unwrap();
        
        let sub = root.join("sub");
        fs::create_dir(&sub).unwrap();
        let file = sub.join("deep.txt");
        File::create(&file).unwrap();

        let result = sanitize_path(&root, "sub/deep.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), file);
    }
    
    #[test]
    fn test_non_existent() {
        let temp = TempDir::new().unwrap();
        let root = temp.path().canonicalize().unwrap();
        
        let result = sanitize_path(&root, "missing.txt");
        // Should be error (NotFound usually)
        assert!(result.is_err());
    }
}
