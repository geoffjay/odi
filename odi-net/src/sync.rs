use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use odi_core::{Issue, IssueId, Remote, IssueStatus, Priority};
use crate::{Result, NetError};
use crate::protocol::{Protocol, ProtocolHandler, HttpsHandler, SshHandler};
use crate::auth::Credential;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncMetadata {
    pub last_modified: DateTime<Utc>,
    pub checksum: String,
    pub size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueMetadata {
    pub id: IssueId,
    pub title: String,
    pub status: IssueStatus,
    pub priority: Priority,
    pub last_modified: DateTime<Utc>,
    pub checksum: String,
}

#[derive(Debug, Clone)]
pub struct SyncClient {
    pub remote_url: String,
    pub protocol: String,
    pub session_id: String,
}

impl SyncClient {
    pub fn new(remote_url: String, protocol: String) -> Self {
        Self {
            remote_url,
            protocol,
            session_id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteSyncState {
    pub remote_name: String,
    pub last_sync: Option<DateTime<Utc>>,
    pub total_issues: u32,
    pub pending_changes: u32,
}

#[async_trait::async_trait]
pub trait RemoteSync: Send + Sync {
    async fn connect(&self, remote: &Remote) -> Result<SyncClient>;
    async fn list_issues(&self, client: &SyncClient) -> Result<Vec<IssueMetadata>>;
    async fn download_issue(&self, client: &SyncClient, id: &IssueId) -> Result<Issue>;
    async fn upload_issue(&self, client: &SyncClient, issue: &Issue) -> Result<()>;
    async fn get_sync_state(&self, client: &SyncClient) -> Result<RemoteSyncState>;
}

pub struct DefaultRemoteSync;

impl DefaultRemoteSync {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl RemoteSync for DefaultRemoteSync {
    async fn connect(&self, remote: &Remote) -> Result<SyncClient> {
        // Parse the remote URL to determine protocol
        let url = url::Url::parse(&remote.url).map_err(|e| NetError::Sync {
            message: format!("Invalid remote URL '{}': {}", remote.url, e),
        })?;

        let protocol = match url.scheme() {
            "ssh" => Protocol::SSH,
            "https" | "http" => Protocol::HTTPS,
            scheme => return Err(NetError::Sync {
                message: format!("Unsupported protocol: {}", scheme),
            }),
        };

        // Create appropriate protocol handler
        let handler: Box<dyn ProtocolHandler> = match protocol {
            Protocol::SSH => Box::new(SshHandler::new()),
            Protocol::HTTPS => Box::new(HttpsHandler::new()),
        };

        // Extract credentials from URL or use defaults
        let credentials = if let Some(password) = url.password() {
            // If password is provided in URL, treat as token
            Credential::Token {
                value: password.to_string(),
            }
        } else {
            // For SSH without credentials in URL, try default SSH key
            if protocol == Protocol::SSH {
                Credential::SshKey {
                    path: std::path::PathBuf::from(format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap_or_default())),
                    passphrase: None,
                }
            } else {
                return Err(NetError::Sync {
                    message: "No credentials provided for HTTPS connection".to_string(),
                });
            }
        };

        // Authenticate
        let _auth_token = handler.authenticate(&credentials).await?;

        Ok(SyncClient::new(
            remote.url.clone(),
            match protocol {
                Protocol::SSH => "ssh".to_string(),
                Protocol::HTTPS => "https".to_string(),
            },
        ))
    }

    async fn list_issues(&self, client: &SyncClient) -> Result<Vec<IssueMetadata>> {
        // For now, return empty list - this would fetch issue metadata from remote
        // In a real implementation, this would make a request to list all issues
        println!("Listing issues from remote: {}", client.remote_url);
        Ok(Vec::new())
    }

    async fn download_issue(&self, client: &SyncClient, id: &IssueId) -> Result<Issue> {
        // For now, return error - this would download a specific issue
        println!("Downloading issue {} from remote: {}", id, client.remote_url);
        Err(NetError::Sync {
            message: format!("Issue {} not found on remote", id),
        })
    }

    async fn upload_issue(&self, client: &SyncClient, issue: &Issue) -> Result<()> {
        // For now, just log - this would upload the issue to remote
        println!("Uploading issue {} to remote: {}", issue.id, client.remote_url);
        Ok(())
    }

    async fn get_sync_state(&self, client: &SyncClient) -> Result<RemoteSyncState> {
        Ok(RemoteSyncState {
            remote_name: client.remote_url.clone(),
            last_sync: Some(chrono::Utc::now()),
            total_issues: 0,
            pending_changes: 0,
        })
    }
}
