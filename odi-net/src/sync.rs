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
            // If password is provided in URL, treat as password auth for SSH
            if protocol == Protocol::SSH {
                Credential::Password {
                    username: url.username().to_string(),
                    password: password.to_string(),
                }
            } else {
                // For HTTPS, treat as token
                Credential::Token {
                    value: password.to_string(),
                }
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
        // Parse remote URL to get connection details
        let url = url::Url::parse(&client.remote_url).map_err(|e| NetError::Sync {
            message: format!("Invalid remote URL '{}': {}", client.remote_url, e),
        })?;
        
        println!("📡 Listing issues from remote: {}", client.remote_url);
        
        // Get protocol and create handler
        let protocol = if url.scheme() == "ssh" {
            Protocol::SSH
        } else if url.scheme() == "https" {
            Protocol::HTTPS
        } else {
            return Err(NetError::Sync {
                message: format!("Unsupported protocol: {}", url.scheme()),
            });
        };

        let handler: Box<dyn ProtocolHandler> = match protocol {
            Protocol::SSH => Box::new(SshHandler::new()),
            Protocol::HTTPS => Box::new(HttpsHandler::new()),
        };

        // List remote issues using protocol handler
        match handler.list_objects(&client.remote_url, "issues").await {
            Ok(object_list) => {
                let mut metadata = Vec::new();
                
                for object_name in object_list {
                    if object_name.ends_with(".bin") {
                        let id_str = object_name.trim_end_matches(".bin");
                        if let Ok(id) = id_str.parse::<IssueId>() {
                            // Download issue metadata via protocol handler  
                            match handler.download_object(&client.remote_url, &format!("issues/{}", object_name)).await {
                                Ok(issue_data) => {
                                    // Deserialize the issue to extract metadata
                                    match bincode::deserialize::<Issue>(&issue_data) {
                                        Ok(issue) => {
                                            metadata.push(IssueMetadata {
                                                id: issue.id,
                                                title: issue.title.clone(),
                                                status: issue.status.clone(),
                                                priority: issue.priority.clone(),
                                                last_modified: issue.updated_at,
                                                checksum: format!("{:x}", md5::compute(&issue_data)),
                                            });
                                        },
                                        Err(e) => {
                                            println!("⚠️  Failed to deserialize issue {}: {}", id, e);
                                        }
                                    }
                                },
                                Err(e) => {
                                    println!("⚠️  Failed to download issue {}: {}", id, e);
                                }
                            }
                        }
                    }
                }
                
                Ok(metadata)
            },
            Err(_) => {
                // Remote might not exist yet or have no issues
                Ok(Vec::new())
            }
        }
    }

    async fn download_issue(&self, client: &SyncClient, id: &IssueId) -> Result<Issue> {
        // Parse remote URL to get connection details  
        let url = url::Url::parse(&client.remote_url).map_err(|e| NetError::Sync {
            message: format!("Invalid remote URL '{}': {}", client.remote_url, e),
        })?;
        
        println!("📥 Downloading issue {} from remote: {}", id, client.remote_url);
        
        // Get protocol and create handler
        let protocol = if url.scheme() == "ssh" {
            Protocol::SSH
        } else if url.scheme() == "https" {
            Protocol::HTTPS
        } else {
            return Err(NetError::Sync {
                message: format!("Unsupported protocol: {}", url.scheme()),
            });
        };

        let handler: Box<dyn ProtocolHandler> = match protocol {
            Protocol::SSH => Box::new(SshHandler::new()),
            Protocol::HTTPS => Box::new(HttpsHandler::new()),
        };

        // Download issue using protocol handler
        let issue_data = handler.download_object(&client.remote_url, &format!("issues/{}.bin", id)).await?;
        
        let issue: Issue = bincode::deserialize(&issue_data).map_err(|e| NetError::Sync {
            message: format!("Failed to deserialize remote issue: {}", e),
        })?;
        
        Ok(issue)
    }

    async fn upload_issue(&self, client: &SyncClient, issue: &Issue) -> Result<()> {
        // Parse remote URL to get connection details
        let url = url::Url::parse(&client.remote_url).map_err(|e| NetError::Sync {
            message: format!("Invalid remote URL '{}': {}", client.remote_url, e),
        })?;
        
        println!("📤 Uploading issue {} to remote: {}", issue.id, client.remote_url);
        
        // Get protocol and create handler
        let protocol = if url.scheme() == "ssh" {
            Protocol::SSH
        } else if url.scheme() == "https" {
            Protocol::HTTPS  
        } else {
            return Err(NetError::Sync {
                message: format!("Unsupported protocol: {}", url.scheme()),
            });
        };

        let handler: Box<dyn ProtocolHandler> = match protocol {
            Protocol::SSH => Box::new(SshHandler::new()),
            Protocol::HTTPS => Box::new(HttpsHandler::new()),
        };

        // Serialize the issue
        let issue_data = bincode::serialize(issue).map_err(|e| NetError::Sync {
            message: format!("Failed to serialize issue: {}", e),
        })?;
        
        // Upload issue using protocol handler
        handler.upload_object(&client.remote_url, &format!("issues/{}.bin", issue.id), &issue_data).await?;
        
        Ok(())
    }

    async fn get_sync_state(&self, client: &SyncClient) -> Result<RemoteSyncState> {
        // Get the list of issues to calculate state
        let issues = self.list_issues(client).await?;
        
        Ok(RemoteSyncState {
            remote_name: client.remote_url.clone(),
            last_sync: Some(chrono::Utc::now()),
            total_issues: issues.len() as u32,
            pending_changes: 0, // Would need to compare with local state to calculate this
        })
    }
}
