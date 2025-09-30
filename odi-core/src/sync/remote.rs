//! Remote entity and operations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::{project::ProjectId, sync::RemoteId};

/// Remote entity for synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Remote {
    pub id: RemoteId,
    pub name: String,
    pub url: String,
    pub projects: Vec<ProjectId>,
    pub last_sync: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Remote URL protocol
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteProtocol {
    Ssh,
    Https,
}

impl Remote {
    /// Create a new remote
    pub fn new(id: RemoteId, name: String, url: String) -> Self {
        Self {
            id,
            name,
            url,
            projects: Vec::new(),
            last_sync: None,
            created_at: Utc::now(),
        }
    }

    /// Validate remote ID (3-50 characters, alphanumeric + ._-)
    pub fn validate_id(id: &str) -> bool {
        id.len() >= 3 && id.len() <= 50 && 
        id.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-')
    }

    /// Validate remote name (1-100 characters)
    pub fn validate_name(name: &str) -> bool {
        !name.is_empty() && name.len() <= 100
    }

    /// Validate remote URL format
    pub fn validate_url(url: &str) -> bool {
        // Basic URL validation - check for protocol and host
        url.starts_with("ssh://") || 
        url.starts_with("https://") || 
        url.starts_with("http://") ||
        url.contains('@') // SSH format: user@host:path
    }

    /// Get remote protocol from URL
    pub fn get_protocol(&self) -> Option<RemoteProtocol> {
        if self.url.starts_with("ssh://") || self.url.contains('@') {
            Some(RemoteProtocol::Ssh)
        } else if self.url.starts_with("https://") || self.url.starts_with("http://") {
            Some(RemoteProtocol::Https)
        } else {
            None
        }
    }

    /// Add project to remote
    pub fn add_project(&mut self, project_id: ProjectId) {
        if !self.projects.contains(&project_id) {
            self.projects.push(project_id);
        }
    }

    /// Remove project from remote
    pub fn remove_project(&mut self, project_id: &ProjectId) {
        self.projects.retain(|id| id != project_id);
    }

    /// Check if remote contains project
    pub fn has_project(&self, project_id: &ProjectId) -> bool {
        self.projects.contains(project_id)
    }

    /// Get project count
    pub fn project_count(&self) -> usize {
        self.projects.len()
    }

    /// Update last sync time
    pub fn update_last_sync(&mut self) {
        self.last_sync = Some(Utc::now());
    }

    /// Check if remote has been synced
    pub fn has_synced(&self) -> bool {
        self.last_sync.is_some()
    }

    /// Get time since last sync
    pub fn time_since_sync(&self) -> Option<chrono::Duration> {
        self.last_sync.map(|last| Utc::now() - last)
    }

    /// Update remote name
    pub fn set_name(&mut self, name: String) -> Result<(), String> {
        if !Self::validate_name(&name) {
            return Err("Invalid remote name".to_string());
        }
        self.name = name;
        Ok(())
    }

    /// Update remote URL
    pub fn set_url(&mut self, url: String) -> Result<(), String> {
        if !Self::validate_url(&url) {
            return Err("Invalid remote URL format".to_string());
        }
        self.url = url;
        Ok(())
    }

    /// Get SSH user from URL (if SSH protocol)
    pub fn get_ssh_user(&self) -> Option<String> {
        if self.get_protocol() == Some(RemoteProtocol::Ssh) && self.url.contains('@') {
            let parts: Vec<&str> = self.url.split('@').collect();
            if parts.len() >= 2 {
                // Extract user from ssh://user@host or user@host
                let user_part = parts[0];
                if let Some(user) = user_part.split("://").last() {
                    return Some(user.to_string());
                }
            }
        }
        None
    }

    /// Get hostname from URL
    pub fn get_hostname(&self) -> Option<String> {
        if let Some(protocol) = self.get_protocol() {
            match protocol {
                RemoteProtocol::Https => {
                    // Extract from https://hostname/path
                    if let Some(start) = self.url.find("://") {
                        let after_protocol = &self.url[start + 3..];
                        if let Some(end) = after_protocol.find('/') {
                            return Some(after_protocol[..end].to_string());
                        } else {
                            return Some(after_protocol.to_string());
                        }
                    }
                }
                RemoteProtocol::Ssh => {
                    if self.url.contains('@') {
                        // Extract from user@hostname:path or ssh://user@hostname/path
                        let parts: Vec<&str> = self.url.split('@').collect();
                        if parts.len() >= 2 {
                            let host_part = parts[1];
                            // Handle both ssh://user@hostname/path and user@hostname:path
                            if let Some(end) = host_part.find(':') {
                                return Some(host_part[..end].to_string());
                            } else if let Some(end) = host_part.find('/') {
                                return Some(host_part[..end].to_string());
                            } else {
                                return Some(host_part.to_string());
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remote_creation() {
        let remote = Remote::new(
            "origin".to_string(),
            "Origin Remote".to_string(),
            "ssh://user@example.com/repo".to_string(),
        );
        
        assert_eq!(remote.id, "origin");
        assert_eq!(remote.name, "Origin Remote");
        assert_eq!(remote.url, "ssh://user@example.com/repo");
        assert!(remote.projects.is_empty());
        assert!(!remote.has_synced());
    }

    #[test]
    fn test_remote_validation() {
        // ID validation
        assert!(Remote::validate_id("origin"));
        assert!(Remote::validate_id("remote-1"));
        assert!(Remote::validate_id("my.remote"));
        assert!(!Remote::validate_id("ab")); // Too short
        assert!(!Remote::validate_id("")); // Empty
        
        let long_id = "a".repeat(51);
        assert!(!Remote::validate_id(&long_id)); // Too long

        // Name validation
        assert!(Remote::validate_name("Origin Remote"));
        assert!(!Remote::validate_name(""));
        
        let long_name = "a".repeat(101);
        assert!(!Remote::validate_name(&long_name));

        // URL validation
        assert!(Remote::validate_url("ssh://user@example.com/repo"));
        assert!(Remote::validate_url("https://github.com/user/repo"));
        assert!(Remote::validate_url("user@example.com:repo"));
        assert!(!Remote::validate_url("invalid-url"));
    }

    #[test]
    fn test_protocol_detection() {
        let ssh_remote = Remote::new(
            "ssh".to_string(),
            "SSH Remote".to_string(),
            "ssh://user@example.com/repo".to_string(),
        );
        assert_eq!(ssh_remote.get_protocol(), Some(RemoteProtocol::Ssh));

        let ssh_shorthand = Remote::new(
            "ssh2".to_string(),
            "SSH Shorthand".to_string(),
            "user@example.com:repo".to_string(),
        );
        assert_eq!(ssh_shorthand.get_protocol(), Some(RemoteProtocol::Ssh));

        let https_remote = Remote::new(
            "https".to_string(),
            "HTTPS Remote".to_string(),
            "https://github.com/user/repo".to_string(),
        );
        assert_eq!(https_remote.get_protocol(), Some(RemoteProtocol::Https));

        let invalid_remote = Remote::new(
            "invalid".to_string(),
            "Invalid Remote".to_string(),
            "ftp://example.com".to_string(),
        );
        assert_eq!(invalid_remote.get_protocol(), None);
    }

    #[test]
    fn test_project_management() {
        let mut remote = Remote::new(
            "origin".to_string(),
            "Origin".to_string(),
            "ssh://user@example.com/repo".to_string(),
        );
        
        remote.add_project("project1".to_string());
        remote.add_project("project2".to_string());
        remote.add_project("project1".to_string()); // Duplicate should be ignored
        
        assert_eq!(remote.project_count(), 2);
        assert!(remote.has_project(&"project1".to_string()));
        assert!(remote.has_project(&"project2".to_string()));
        assert!(!remote.has_project(&"project3".to_string()));
        
        remote.remove_project(&"project1".to_string());
        assert_eq!(remote.project_count(), 1);
        assert!(!remote.has_project(&"project1".to_string()));
    }

    #[test]
    fn test_sync_tracking() {
        let mut remote = Remote::new(
            "origin".to_string(),
            "Origin".to_string(),
            "ssh://user@example.com/repo".to_string(),
        );
        
        assert!(!remote.has_synced());
        assert!(remote.time_since_sync().is_none());
        
        remote.update_last_sync();
        assert!(remote.has_synced());
        assert!(remote.time_since_sync().is_some());
    }

    #[test]
    fn test_url_parsing() {
        let ssh_remote = Remote::new(
            "ssh".to_string(),
            "SSH".to_string(),
            "ssh://git@github.com/user/repo".to_string(),
        );
        assert_eq!(ssh_remote.get_ssh_user(), Some("git".to_string()));
        assert_eq!(ssh_remote.get_hostname(), Some("github.com".to_string()));

        let ssh_shorthand = Remote::new(
            "ssh2".to_string(),
            "SSH2".to_string(),
            "git@github.com:user/repo".to_string(),
        );
        assert_eq!(ssh_shorthand.get_ssh_user(), Some("git".to_string()));
        assert_eq!(ssh_shorthand.get_hostname(), Some("github.com".to_string()));

        let https_remote = Remote::new(
            "https".to_string(),
            "HTTPS".to_string(),
            "https://github.com/user/repo".to_string(),
        );
        assert_eq!(https_remote.get_hostname(), Some("github.com".to_string()));
        assert_eq!(https_remote.get_ssh_user(), None);
    }
}