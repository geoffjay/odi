//! Remote synchronization

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::protocols::ProtocolHandler;
use crate::Result;
use odi_core::{Issue, IssueId, Remote};

/// Sync client for remote operations
pub struct SyncClient {
    pub protocol: Box<dyn ProtocolHandler>,
    pub authenticated: bool,
    pub base_url: String,
}

/// Issue metadata for sync operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueMetadata {
    pub id: IssueId,
    pub last_modified: DateTime<Utc>,
    pub checksum: String,
}

/// Remote sync state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteSyncState {
    pub last_sync: Option<DateTime<Utc>>,
    pub total_issues: usize,
    pub pending_changes: usize,
}

/// Remote synchronization trait
#[async_trait::async_trait]
pub trait RemoteSync {
    async fn connect(&self, remote: &Remote) -> Result<SyncClient>;
    async fn list_issues(&self, client: &SyncClient) -> Result<Vec<IssueMetadata>>;
    async fn download_issue(&self, client: &SyncClient, id: &IssueId) -> Result<Issue>;
    async fn upload_issue(&self, client: &SyncClient, issue: &Issue) -> Result<()>;
    async fn get_sync_state(&self, client: &SyncClient) -> Result<RemoteSyncState>;
}