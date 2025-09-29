//! Authentication and credential management

use std::path::PathBuf;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::Result;
use odi_core::Remote;

/// Authentication token
#[derive(Debug, Clone)]
pub struct AuthToken {
    pub token: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub refresh_token: Option<String>,
}

/// Credential types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Credential {
    SshKey { path: PathBuf, passphrase: Option<String> },
    Token { value: String },
    OAuth { client_id: String, refresh_token: String },
}

/// Authentication trait
#[async_trait::async_trait]
pub trait Authentication {
    async fn validate_credential(&self, credential: &Credential) -> Result<bool>;
    async fn refresh_token(&self, auth: &AuthToken) -> Result<AuthToken>;
    async fn revoke_token(&self, auth: &AuthToken) -> Result<()>;
    
    fn load_credential(&self, remote: &Remote) -> Result<Option<Credential>>;
    fn store_credential(&self, remote: &Remote, credential: &Credential) -> Result<()>;
    fn remove_credential(&self, remote: &Remote) -> Result<()>;
}