use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use odi_core::{Issue, IssueId, Remote, IssueStatus, Priority};
use crate::{Result, NetError};

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
    async fn connect(&self, _remote: &Remote) -> Result<SyncClient> {
        Err(NetError::Sync {
            message: "RemoteSync::connect not implemented yet".to_string(),
        })
    }

    async fn list_issues(&self, _client: &SyncClient) -> Result<Vec<IssueMetadata>> {
        Err(NetError::Sync {
            message: "RemoteSync::list_issues not implemented yet".to_string(),
        })
    }

    async fn download_issue(&self, _client: &SyncClient, _id: &IssueId) -> Result<Issue> {
        Err(NetError::Sync {
            message: "RemoteSync::download_issue not implemented yet".to_string(),
        })
    }

    async fn upload_issue(&self, _client: &SyncClient, _issue: &Issue) -> Result<()> {
        Err(NetError::Sync {
            message: "RemoteSync::upload_issue not implemented yet".to_string(),
        })
    }

    async fn get_sync_state(&self, _client: &SyncClient) -> Result<RemoteSyncState> {
        Err(NetError::Sync {
            message: "RemoteSync::get_sync_state not implemented yet".to_string(),
        })
    }
}
