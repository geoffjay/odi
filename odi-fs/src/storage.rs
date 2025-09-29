use std::path::Path;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

use crate::Result;

/// Object hash type (SHA-256)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectHash(pub String);

/// Object type enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectType {
    Issue,
    User,
    Team,
    Project,
    Workspace,
    Remote,
    Label,
}

/// Storage engine trait
#[async_trait::async_trait]
pub trait StorageEngine {
    async fn initialize(&self, path: &Path) -> Result<()>;
    
    // Object store operations (Git-like)
    async fn write_object<T: Serialize + Send + Sync>(&self, obj: &T) -> Result<ObjectHash>;
    async fn read_object<T: for<'de> Deserialize<'de>>(&self, hash: &ObjectHash) -> Result<Option<T>>;
    async fn delete_object(&self, hash: &ObjectHash) -> Result<()>;
    async fn list_objects(&self, object_type: ObjectType) -> Result<Vec<ObjectHash>>;
    
    // Reference operations
    async fn write_ref(&self, name: &str, hash: &ObjectHash) -> Result<()>;
    async fn read_ref(&self, name: &str) -> Result<Option<ObjectHash>>;
    async fn list_refs(&self, prefix: &str) -> Result<Vec<String>>;
    
    // Locking operations
    async fn lock(&self, resource: &str) -> Result<Lock>;
    async fn unlock(&self, lock: Lock) -> Result<()>;
}

/// File lock for concurrent access
#[derive(Debug)]
pub struct Lock {
    pub resource: String,
    pub acquired_at: DateTime<Utc>,
    pub lock_file: std::path::PathBuf,
}

impl std::fmt::Display for ObjectHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ObjectHash {
    /// Create hash from content
    pub fn from_content(content: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(content);
        let result = hasher.finalize();
        ObjectHash(hex::encode(result))
    }

    /// Validate hash format
    pub fn is_valid(&self) -> bool {
        self.0.len() == 64 && self.0.chars().all(|c| c.is_ascii_hexdigit())
    }
}

/// Default filesystem storage implementation
pub struct FilesystemStorage {
    base_path: std::path::PathBuf,
}

impl FilesystemStorage {
    pub fn new(base_path: std::path::PathBuf) -> Self {
        Self { base_path }
    }

    fn objects_dir(&self) -> std::path::PathBuf {
        self.base_path.join("objects")
    }

    fn refs_dir(&self) -> std::path::PathBuf {
        self.base_path.join("refs")
    }

    fn hash_to_path(&self, hash: &ObjectHash) -> std::path::PathBuf {
        let hash_str = &hash.0;
        self.objects_dir()
            .join(&hash_str[0..2])
            .join(&hash_str[2..])
    }
}

#[async_trait::async_trait]
impl StorageEngine for FilesystemStorage {
    async fn initialize(&self, path: &Path) -> Result<()> {
        let odi_dir = path.join(".odi");
        tokio::fs::create_dir_all(&odi_dir).await?;
        tokio::fs::create_dir_all(odi_dir.join("objects")).await?;
        tokio::fs::create_dir_all(odi_dir.join("refs")).await?;
        Ok(())
    }

    async fn write_object<T: Serialize + Send + Sync>(&self, obj: &T) -> Result<ObjectHash> {
        let content = serde_json::to_vec(obj)?;
        let hash = ObjectHash::from_content(&content);
        
        let obj_path = self.hash_to_path(&hash);
        if let Some(parent) = obj_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        // Use compression for storage efficiency
        let compressed = {
            use flate2::write::GzEncoder;
            use flate2::Compression;
            use std::io::Write;
            
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&content)?;
            encoder.finish()?
        };
        
        tokio::fs::write(obj_path, compressed).await?;
        Ok(hash)
    }

    async fn read_object<T: for<'de> Deserialize<'de>>(&self, hash: &ObjectHash) -> Result<Option<T>> {
        let obj_path = self.hash_to_path(hash);
        
        if !obj_path.exists() {
            return Ok(None);
        }
        
        let compressed = tokio::fs::read(obj_path).await?;
        
        // Decompress content
        let content = {
            use flate2::read::GzDecoder;
            use std::io::Read;
            
            let mut decoder = GzDecoder::new(&compressed[..]);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed)?;
            decompressed
        };
        
        let obj = serde_json::from_slice(&content)?;
        Ok(Some(obj))
    }

    async fn delete_object(&self, hash: &ObjectHash) -> Result<()> {
        let obj_path = self.hash_to_path(hash);
        if obj_path.exists() {
            tokio::fs::remove_file(obj_path).await?;
        }
        Ok(())
    }

    async fn list_objects(&self, _object_type: ObjectType) -> Result<Vec<ObjectHash>> {
        let objects_dir = self.objects_dir();
        let mut hashes = Vec::new();
        
        if !objects_dir.exists() {
            return Ok(hashes);
        }
        
        let mut entries = tokio::fs::read_dir(&objects_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_dir() {
                let subdir_path = entry.path();
                let prefix = entry.file_name().to_string_lossy().to_string();
                
                let mut subentries = tokio::fs::read_dir(&subdir_path).await?;
                while let Some(subentry) = subentries.next_entry().await? {
                    if subentry.file_type().await?.is_file() {
                        let suffix = subentry.file_name().to_string_lossy().to_string();
                        let hash = ObjectHash(format!("{}{}", prefix, suffix));
                        if hash.is_valid() {
                            hashes.push(hash);
                        }
                    }
                }
            }
        }
        
        Ok(hashes)
    }

    async fn write_ref(&self, name: &str, hash: &ObjectHash) -> Result<()> {
        let ref_path = self.refs_dir().join(name);
        if let Some(parent) = ref_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(ref_path, hash.0.as_bytes()).await?;
        Ok(())
    }

    async fn read_ref(&self, name: &str) -> Result<Option<ObjectHash>> {
        let ref_path = self.refs_dir().join(name);
        if !ref_path.exists() {
            return Ok(None);
        }
        
        let content = tokio::fs::read_to_string(ref_path).await?;
        let hash = ObjectHash(content.trim().to_string());
        
        if hash.is_valid() {
            Ok(Some(hash))
        } else {
            Ok(None)
        }
    }

    async fn list_refs(&self, prefix: &str) -> Result<Vec<String>> {
        let refs_dir = self.refs_dir();
        let prefix_path = refs_dir.join(prefix);
        let mut refs = Vec::new();
        
        if !prefix_path.exists() {
            return Ok(refs);
        }
        
        // Simple non-recursive implementation to avoid boxing issues
        let mut entries = tokio::fs::read_dir(&prefix_path).await?;
        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_file() {
                if let Ok(relative) = entry.path().strip_prefix(&refs_dir) {
                    if let Some(ref_name) = relative.to_str() {
                        refs.push(ref_name.to_string());
                    }
                }
            }
        }
        
        Ok(refs)
    }

    async fn lock(&self, resource: &str) -> Result<Lock> {
        let lock_file = self.base_path.join("locks").join(format!("{}.lock", resource));
        if let Some(parent) = lock_file.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        // Simple file-based locking
        tokio::fs::write(&lock_file, "locked").await?;
        
        Ok(Lock {
            resource: resource.to_string(),
            acquired_at: Utc::now(),
            lock_file,
        })
    }

    async fn unlock(&self, lock: Lock) -> Result<()> {
        if lock.lock_file.exists() {
            tokio::fs::remove_file(lock.lock_file).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use odi_core::User;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_storage_initialization() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let storage = FilesystemStorage::new(temp_dir.path().to_path_buf());
        
        storage.initialize(temp_dir.path()).await.expect("Should initialize");
        
        assert!(temp_dir.path().join(".odi").exists());
        assert!(temp_dir.path().join(".odi/objects").exists());
        assert!(temp_dir.path().join(".odi/refs").exists());
    }

    #[tokio::test]
    async fn test_object_storage_operations() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let storage = FilesystemStorage::new(temp_dir.path().join(".odi"));
        storage.initialize(temp_dir.path()).await.expect("Should initialize");
        
        let user = User::new(
            "test_user".to_string(),
            "Test User".to_string(),
            "test@example.com".to_string(),
        );
        
        // Write object
        let hash = storage.write_object(&user).await.expect("Should write object");
        assert!(!hash.0.is_empty());
        assert!(hash.is_valid());
        
        // Read object back
        let read_user: Option<User> = storage.read_object(&hash).await.expect("Should read object");
        assert!(read_user.is_some());
        
        let read_user = read_user.unwrap();
        assert_eq!(read_user.id, user.id);
        assert_eq!(read_user.name, user.name);
    }

    #[tokio::test]
    async fn test_content_addressed_storage() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let storage = FilesystemStorage::new(temp_dir.path().join(".odi"));
        storage.initialize(temp_dir.path()).await.expect("Should initialize");
        
        // Create users with same content but handle timestamps
        let mut user1 = User::new("same".to_string(), "Same User".to_string(), "same@example.com".to_string());
        let mut user2 = User::new("same".to_string(), "Same User".to_string(), "same@example.com".to_string());
        
        // Make timestamps identical to ensure same hash
        let fixed_time = chrono::Utc::now();
        user1.created_at = fixed_time;
        user1.last_active = fixed_time;
        user2.created_at = fixed_time;
        user2.last_active = fixed_time;
        
        let hash1 = storage.write_object(&user1).await.expect("Should write user1");
        let hash2 = storage.write_object(&user2).await.expect("Should write user2");
        
        // Identical content should produce identical hashes (deduplication)
        assert_eq!(hash1, hash2);
        
        // Different content should produce different hashes
        let user3 = User::new("different".to_string(), "Different User".to_string(), "different@example.com".to_string());
        let hash3 = storage.write_object(&user3).await.expect("Should write user3");
        
        assert_ne!(hash1, hash3);
    }

    #[tokio::test]
    async fn test_reference_operations() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let storage = FilesystemStorage::new(temp_dir.path().join(".odi"));
        storage.initialize(temp_dir.path()).await.expect("Should initialize");
        
        let user = User::new("ref_user".to_string(), "Reference User".to_string(), "ref@example.com".to_string());
        let hash = storage.write_object(&user).await.expect("Should write user");
        
        // Write reference
        storage.write_ref("users/ref_user", &hash).await.expect("Should write ref");
        
        // Read reference
        let ref_hash = storage.read_ref("users/ref_user").await.expect("Should read ref");
        assert!(ref_hash.is_some());
        assert_eq!(ref_hash.unwrap(), hash);
    }
}