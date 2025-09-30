use serde::{Deserialize, Serialize};
use crate::{Result, NetError, AuthToken};
use serde_json;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Protocol {
    SSH,
    HTTPS,
}

#[async_trait::async_trait]
pub trait ProtocolHandler: Send + Sync {
    async fn authenticate(&self, credentials: &crate::Credential) -> Result<AuthToken>;
    async fn get(&self, path: &str, auth: &AuthToken) -> Result<Vec<u8>>;
    async fn post(&self, path: &str, data: &[u8], auth: &AuthToken) -> Result<Vec<u8>>;
    async fn put(&self, path: &str, data: &[u8], auth: &AuthToken) -> Result<Vec<u8>>;
    async fn delete(&self, path: &str, auth: &AuthToken) -> Result<()>;
}

pub struct HttpsHandler;
pub struct SshHandler;

impl HttpsHandler {
    pub fn new() -> Self {
        Self
    }
}

impl SshHandler {
    pub fn new() -> Self {
        Self
    }

    async fn ssh_operation(&self, path: &str, data: Option<&[u8]>, operation: &str, auth: &AuthToken) -> Result<Vec<u8>> {
        // Parse the SSH URL to extract host, port, user, and path
        let url = url::Url::parse(&format!("ssh://{}", path))
            .map_err(|e| NetError::Protocol {
                message: format!("Invalid SSH URL: {}", e),
            })?;

        let host = url.host_str().ok_or_else(|| NetError::Protocol {
            message: "No host in SSH URL".to_string(),
        })?;
        
        let port = url.port().unwrap_or(22);
        let remote_path = url.path();
        let username = url.username();
        
        if username.is_empty() {
            return Err(NetError::Protocol {
                message: "No username in SSH URL".to_string(),
            });
        }

        // Parse auth token to get credentials
        let auth_data: serde_json::Value = serde_json::from_str(&auth.token)
            .map_err(|e| NetError::Protocol {
                message: format!("Invalid auth token: {}", e),
            })?;

        // For now, we'll use a simple approach with std::process to execute SSH commands
        // This is a basic implementation that works with our Docker setup
        match operation {
            "get" => {
                let output = tokio::process::Command::new("ssh")
                    .arg("-p").arg(port.to_string())
                    .arg("-o").arg("StrictHostKeyChecking=no")
                    .arg("-o").arg("UserKnownHostsFile=/dev/null")
                    .arg(format!("{}@{}", username, host))
                    .arg(format!("cat {}", remote_path))
                    .output()
                    .await
                    .map_err(|e| NetError::Protocol {
                        message: format!("SSH command failed: {}", e),
                    })?;

                if !output.status.success() {
                    return Err(NetError::Protocol {
                        message: format!("SSH get failed: {}", String::from_utf8_lossy(&output.stderr)),
                    });
                }

                Ok(output.stdout)
            },
            "post" | "put" => {
                let data = data.ok_or_else(|| NetError::Protocol {
                    message: "No data provided for SSH post/put operation".to_string(),
                })?;

                // For put operations, we'll use scp to transfer the file
                let temp_file = tempfile::NamedTempFile::new()
                    .map_err(|e| NetError::Protocol {
                        message: format!("Failed to create temp file: {}", e),
                    })?;

                tokio::fs::write(temp_file.path(), data).await
                    .map_err(|e| NetError::Protocol {
                        message: format!("Failed to write temp file: {}", e),
                    })?;

                let output = tokio::process::Command::new("scp")
                    .arg("-P").arg(port.to_string())
                    .arg("-o").arg("StrictHostKeyChecking=no")
                    .arg("-o").arg("UserKnownHostsFile=/dev/null")
                    .arg(temp_file.path())
                    .arg(format!("{}@{}:{}", username, host, remote_path))
                    .output()
                    .await
                    .map_err(|e| NetError::Protocol {
                        message: format!("SCP command failed: {}", e),
                    })?;

                if !output.status.success() {
                    return Err(NetError::Protocol {
                        message: format!("SSH put failed: {}", String::from_utf8_lossy(&output.stderr)),
                    });
                }

                Ok(Vec::new()) // Success, no data returned
            },
            "delete" => {
                let output = tokio::process::Command::new("ssh")
                    .arg("-p").arg(port.to_string())
                    .arg("-o").arg("StrictHostKeyChecking=no")
                    .arg("-o").arg("UserKnownHostsFile=/dev/null")
                    .arg(format!("{}@{}", username, host))
                    .arg(format!("rm -f {}", remote_path))
                    .output()
                    .await
                    .map_err(|e| NetError::Protocol {
                        message: format!("SSH command failed: {}", e),
                    })?;

                if !output.status.success() {
                    return Err(NetError::Protocol {
                        message: format!("SSH delete failed: {}", String::from_utf8_lossy(&output.stderr)),
                    });
                }

                Ok(Vec::new())
            },
            _ => Err(NetError::Protocol {
                message: format!("Unknown SSH operation: {}", operation),
            })
        }
    }
}

#[async_trait::async_trait]
impl ProtocolHandler for HttpsHandler {
    async fn authenticate(&self, credentials: &crate::Credential) -> Result<AuthToken> {
        use base64::Engine;
        
        // Create basic auth header for HTTPS
        let auth_header = match credentials {
            crate::Credential::Token { value } => {
                format!("Bearer {}", value)
            },
            _ => return Err(NetError::Protocol {
                message: "HTTPS requires token authentication".to_string(),
            }),
        };
        
        Ok(AuthToken {
            token: auth_header,
            expires_at: None,
            refresh_token: None,
        })
    }

    async fn get(&self, path: &str, auth: &AuthToken) -> Result<Vec<u8>> {
        let client = reqwest::Client::new();
        let response = client
            .get(path)
            .header("Authorization", &auth.token)
            .send()
            .await
            .map_err(|e| NetError::Protocol {
                message: format!("HTTPS GET failed: {}", e),
            })?;
        
        let bytes = response.bytes().await.map_err(|e| NetError::Protocol {
            message: format!("Failed to read HTTPS response: {}", e),
        })?;
        
        Ok(bytes.to_vec())
    }

    async fn post(&self, path: &str, data: &[u8], auth: &AuthToken) -> Result<Vec<u8>> {
        let client = reqwest::Client::new();
        let response = client
            .post(path)
            .header("Authorization", &auth.token)
            .header("Content-Type", "application/octet-stream")
            .body(data.to_vec())
            .send()
            .await
            .map_err(|e| NetError::Protocol {
                message: format!("HTTPS POST failed: {}", e),
            })?;
        
        let bytes = response.bytes().await.map_err(|e| NetError::Protocol {
            message: format!("Failed to read HTTPS response: {}", e),
        })?;
        
        Ok(bytes.to_vec())
    }

    async fn put(&self, path: &str, data: &[u8], auth: &AuthToken) -> Result<Vec<u8>> {
        let client = reqwest::Client::new();
        let response = client
            .put(path)
            .header("Authorization", &auth.token)
            .header("Content-Type", "application/octet-stream")
            .body(data.to_vec())
            .send()
            .await
            .map_err(|e| NetError::Protocol {
                message: format!("HTTPS PUT failed: {}", e),
            })?;
        
        let bytes = response.bytes().await.map_err(|e| NetError::Protocol {
            message: format!("Failed to read HTTPS response: {}", e),
        })?;
        
        Ok(bytes.to_vec())
    }

    async fn delete(&self, path: &str, auth: &AuthToken) -> Result<()> {
        let client = reqwest::Client::new();
        let _response = client
            .delete(path)
            .header("Authorization", &auth.token)
            .send()
            .await
            .map_err(|e| NetError::Protocol {
                message: format!("HTTPS DELETE failed: {}", e),
            })?;
        
        Ok(())
    }
}

#[async_trait::async_trait]
impl ProtocolHandler for SshHandler {
    async fn authenticate(&self, credentials: &crate::Credential) -> Result<AuthToken> {
        // For SSH, we'll create a token that contains the connection info
        match credentials {
            crate::Credential::SshKey { path, passphrase } => {
                let token_data = serde_json::json!({
                    "type": "ssh_key",
                    "path": path.to_string_lossy(),
                    "passphrase": passphrase.as_deref().unwrap_or("")
                });
                Ok(AuthToken {
                    token: token_data.to_string(),
                    expires_at: None,
                    refresh_token: None,
                })
            },
            _ => Err(NetError::Protocol {
                message: "SSH requires SSH key authentication".to_string(),
            }),
        }
    }

    async fn get(&self, path: &str, auth: &AuthToken) -> Result<Vec<u8>> {
        self.ssh_operation(path, None, "get", auth).await
    }

    async fn post(&self, path: &str, data: &[u8], auth: &AuthToken) -> Result<Vec<u8>> {
        self.ssh_operation(path, Some(data), "post", auth).await
    }

    async fn put(&self, path: &str, data: &[u8], auth: &AuthToken) -> Result<Vec<u8>> {
        self.ssh_operation(path, Some(data), "put", auth).await
    }

    async fn delete(&self, path: &str, auth: &AuthToken) -> Result<()> {
        self.ssh_operation(path, None, "delete", auth).await?;
        Ok(())
    }
}