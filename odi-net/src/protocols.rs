//! Protocol handlers for SSH and HTTPS

use serde::{Deserialize, Serialize};

use crate::auth::{AuthToken, Credential};
use crate::Result;

/// Protocol enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Protocol {
    SSH,
    HTTPS,
}

/// Protocol handler trait
#[async_trait::async_trait]
pub trait ProtocolHandler: Send + Sync {
    async fn authenticate(&self, credential: &Credential) -> Result<AuthToken>;
    async fn get(&self, path: &str, auth: &AuthToken) -> Result<Vec<u8>>;
    async fn post(&self, path: &str, data: &[u8], auth: &AuthToken) -> Result<Vec<u8>>;
    async fn put(&self, path: &str, data: &[u8], auth: &AuthToken) -> Result<Vec<u8>>;
    async fn delete(&self, path: &str, auth: &AuthToken) -> Result<()>;
}