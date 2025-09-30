use crate::Result;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};

/// Object type in the storage system
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ObjectType {
    Issue,
    User,
    Team,
    Project,
    Label,
    Remote,
}

/// Storage object with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageObject {
    pub object_type: ObjectType,
    pub hash: String,
    pub size: usize,
    pub compressed_size: usize,
    pub data: Vec<u8>,
}

/// Reference to an object (symbolic pointer)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectRef {
    pub name: String,
    pub hash: String,
    pub object_type: ObjectType,
}

/// Lock for concurrent access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLock {
    pub name: String,
    pub acquired_at: DateTime<Utc>,
    pub process_id: u32,
}

/// Object hash type for content addressing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ObjectHash(String);

impl ObjectHash {
    pub fn new(hash: String) -> Self {
        Self(hash)
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for ObjectHash {
    fn from(hash: String) -> Self {
        Self(hash)
    }
}

impl From<&str> for ObjectHash {
    fn from(hash: &str) -> Self {
        Self(hash.to_string())
    }
}

/// Lock handle for storage operations
pub type Lock = StorageLock;

pub trait ObjectStorage {
    fn store_object(&self, object_type: ObjectType, data: &[u8]) -> Result<String>;
    fn retrieve_object(&self, hash: &str) -> Result<Option<StorageObject>>;
    fn delete_object(&self, hash: &str) -> Result<bool>;
    fn list_objects(&self, filter_type: Option<ObjectType>) -> Result<Vec<String>>;
    fn object_exists(&self, hash: &str) -> Result<bool>;
    fn get_object_hash(data: &[u8]) -> String;
    
    // Reference operations
    fn create_ref(&self, name: &str, hash: &str, object_type: ObjectType) -> Result<()>;
    fn get_ref(&self, name: &str) -> Result<Option<ObjectRef>>;
    fn delete_ref(&self, name: &str) -> Result<bool>;
    fn list_refs(&self) -> Result<Vec<ObjectRef>>;
    
    // Locking operations
    fn acquire_lock(&self, name: &str) -> Result<StorageLock>;
    fn release_lock(&self, lock: &StorageLock) -> Result<()>;
    fn is_locked(&self, name: &str) -> Result<bool>;
}

/// High-level storage engine interface for Git-like operations
#[async_trait::async_trait]
pub trait StorageEngine {
    async fn initialize(&self, path: &Path) -> Result<()>;
    async fn write_object<T: serde::Serialize + Send + Sync>(&self, obj: &T) -> Result<ObjectHash>;
    async fn read_object<T: serde::de::DeserializeOwned + Send>(&self, hash: &ObjectHash) -> Result<Option<T>>;
    async fn delete_object(&self, hash: &ObjectHash) -> Result<()>;
    async fn list_objects(&self, object_type: ObjectType) -> Result<Vec<ObjectHash>>;
    async fn write_ref(&self, name: &str, hash: &ObjectHash) -> Result<()>;
    async fn read_ref(&self, name: &str) -> Result<Option<ObjectHash>>;
    async fn list_refs(&self, prefix: &str) -> Result<Vec<String>>;
    async fn lock(&self, resource: &str) -> Result<Lock>;
    async fn unlock(&self, lock: Lock) -> Result<()>;
}

#[derive(Clone)]
pub struct FileSystemStorage {
    root_path: PathBuf,
    objects_path: PathBuf,
    refs_path: PathBuf,
    locks_path: PathBuf,
}

impl FileSystemStorage {
    pub fn new(root_path: PathBuf) -> Result<Self> {
        let objects_path = root_path.join("objects");
        let refs_path = root_path.join("refs");
        let locks_path = root_path.join("locks");
        
        // Create directories if they don't exist
        fs::create_dir_all(&objects_path)?;
        fs::create_dir_all(&refs_path)?;
        fs::create_dir_all(&locks_path)?;
        
        Ok(Self {
            root_path,
            objects_path,
            refs_path,
            locks_path,
        })
    }
    
    /// Initialize storage in .odi/objects directory
    pub fn init() -> Result<Self> {
        let root_path = PathBuf::from(".odi");
        fs::create_dir_all(&root_path)?;
        Self::new(root_path)
    }
    
    fn get_object_path(&self, hash: &str) -> PathBuf {
        // Split hash into directory structure: first 2 chars as dir, rest as filename
        let (dir, file) = hash.split_at(2);
        self.objects_path.join(dir).join(file)
    }
    
    fn compress_data(data: &[u8]) -> Result<Vec<u8>> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)?;
        Ok(encoder.finish()?)
    }
    
    fn decompress_data(data: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = GzDecoder::new(data);
        let mut result = Vec::new();
        decoder.read_to_end(&mut result)?;
        Ok(result)
    }
}

impl ObjectStorage for FileSystemStorage {
    fn store_object(&self, object_type: ObjectType, data: &[u8]) -> Result<String> {
        let hash = Self::get_object_hash(data);
        let object_path = self.get_object_path(&hash);
        
        // Don't store if already exists
        if object_path.exists() {
            return Ok(hash);
        }
        
        // Create parent directory
        if let Some(parent) = object_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Compress and store
        let compressed_data = Self::compress_data(data)?;
        let storage_object = StorageObject {
            object_type,
            hash: hash.clone(),
            size: data.len(),
            compressed_size: compressed_data.len(),
            data: compressed_data,
        };
        
        let serialized = bincode::serialize(&storage_object).map_err(|e| {
            crate::FsError::SerializationError { message: e.to_string() }
        })?;
        
        fs::write(object_path, serialized)?;
        Ok(hash)
    }
    
    fn retrieve_object(&self, hash: &str) -> Result<Option<StorageObject>> {
        let object_path = self.get_object_path(hash);
        
        if !object_path.exists() {
            return Ok(None);
        }
        
        let serialized = fs::read(object_path)?;
        let mut storage_object: StorageObject = bincode::deserialize(&serialized).map_err(|e| {
            crate::FsError::SerializationError { message: e.to_string() }
        })?;
        
        // Decompress the data
        storage_object.data = Self::decompress_data(&storage_object.data)?;
        Ok(Some(storage_object))
    }
    
    fn delete_object(&self, hash: &str) -> Result<bool> {
        let object_path = self.get_object_path(hash);
        
        if object_path.exists() {
            fs::remove_file(object_path)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    fn list_objects(&self, filter_type: Option<ObjectType>) -> Result<Vec<String>> {
        let mut objects = Vec::new();
        
        // Traverse objects directory
        for entry in fs::read_dir(&self.objects_path)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let dir_name = entry.file_name().to_string_lossy().to_string();
                for file_entry in fs::read_dir(entry.path())? {
                    let file_entry = file_entry?;
                    if file_entry.file_type()?.is_file() {
                        let file_name = file_entry.file_name().to_string_lossy().to_string();
                        let hash = format!("{}{}", dir_name, file_name);
                        
                        // Filter by type if specified
                        if let Some(filter) = filter_type {
                            if let Ok(Some(obj)) = self.retrieve_object(&hash) {
                                if obj.object_type == filter {
                                    objects.push(hash);
                                }
                            }
                        } else {
                            objects.push(hash);
                        }
                    }
                }
            }
        }
        
        Ok(objects)
    }
    
    fn object_exists(&self, hash: &str) -> Result<bool> {
        Ok(self.get_object_path(hash).exists())
    }
    
    fn get_object_hash(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
    
    fn create_ref(&self, name: &str, hash: &str, object_type: ObjectType) -> Result<()> {
        let ref_path = self.refs_path.join(name);
        
        // Create parent directories if they don't exist
        if let Some(parent) = ref_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let object_ref = ObjectRef {
            name: name.to_string(),
            hash: hash.to_string(),
            object_type,
        };
        
        let serialized = bincode::serialize(&object_ref).map_err(|e| {
            crate::FsError::SerializationError { message: e.to_string() }
        })?;
        
        fs::write(ref_path, serialized)?;
        Ok(())
    }
    
    fn get_ref(&self, name: &str) -> Result<Option<ObjectRef>> {
        let ref_path = self.refs_path.join(name);
        
        if !ref_path.exists() {
            return Ok(None);
        }
        
        let serialized = fs::read(ref_path)?;
        let object_ref: ObjectRef = bincode::deserialize(&serialized).map_err(|e| {
            crate::FsError::SerializationError { message: e.to_string() }
        })?;
        
        Ok(Some(object_ref))
    }
    
    fn delete_ref(&self, name: &str) -> Result<bool> {
        let ref_path = self.refs_path.join(name);
        
        if ref_path.exists() {
            fs::remove_file(ref_path)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    fn list_refs(&self) -> Result<Vec<ObjectRef>> {
        let mut refs = Vec::new();
        
        for entry in fs::read_dir(&self.refs_path)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let file_name = entry.file_name().to_string_lossy().to_string();
                if let Ok(Some(object_ref)) = self.get_ref(&file_name) {
                    refs.push(object_ref);
                }
            }
        }
        
        Ok(refs)
    }
    
    fn acquire_lock(&self, name: &str) -> Result<StorageLock> {
        let lock_path = self.locks_path.join(format!("{}.lock", name));
        
        // Check if already locked
        if lock_path.exists() {
            return Err(crate::FsError::LockError {
                message: format!("Resource '{}' is already locked", name),
            });
        }
        
        let lock = StorageLock {
            name: name.to_string(),
            acquired_at: Utc::now(),
            process_id: std::process::id(),
        };
        
        let serialized = bincode::serialize(&lock).map_err(|e| {
            crate::FsError::SerializationError { message: e.to_string() }
        })?;
        
        // Use exclusive create to ensure atomicity
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&lock_path)?;
        file.write_all(&serialized)?;
        
        Ok(lock)
    }
    
    fn release_lock(&self, lock: &StorageLock) -> Result<()> {
        let lock_path = self.locks_path.join(format!("{}.lock", lock.name));
        
        if lock_path.exists() {
            // Verify this is our lock
            let serialized = fs::read(&lock_path)?;
            let existing_lock: StorageLock = bincode::deserialize(&serialized).map_err(|e| {
                crate::FsError::SerializationError { message: e.to_string() }
            })?;
            
            if existing_lock.process_id != lock.process_id || existing_lock.acquired_at != lock.acquired_at {
                return Err(crate::FsError::LockError {
                    message: "Cannot release lock owned by different process".to_string(),
                });
            }
            
            fs::remove_file(lock_path)?;
        }
        
        Ok(())
    }
    
    fn is_locked(&self, name: &str) -> Result<bool> {
        let lock_path = self.locks_path.join(format!("{}.lock", name));
        Ok(lock_path.exists())
    }
}