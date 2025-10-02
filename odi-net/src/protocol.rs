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
    
    // Object-level operations for sync
    async fn list_objects(&self, base_url: &str, path: &str) -> Result<Vec<String>>;
    async fn download_object(&self, base_url: &str, object_path: &str) -> Result<Vec<u8>>;
    async fn upload_object(&self, base_url: &str, object_path: &str, data: &[u8]) -> Result<()>;
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

        let auth_type = auth_data["type"].as_str().unwrap_or("ssh_key");

        // For now, we'll use a simple approach with std::process to execute SSH commands
        // This is a basic implementation that works with our Docker setup
        match operation {
            "get" => {
                let mut cmd = if auth_type == "password" {
                    let password = auth_data["password"].as_str().unwrap_or("");
                    let mut sshpass_cmd = tokio::process::Command::new("sshpass");
                    sshpass_cmd.arg("-p").arg(password)
                              .arg("ssh")
                              .arg("-p").arg(port.to_string())
                              .arg("-o").arg("StrictHostKeyChecking=no")
                              .arg("-o").arg("UserKnownHostsFile=/dev/null")
                              .arg(format!("{}@{}", username, host))
                              .arg(format!("cat {}", remote_path));
                    sshpass_cmd
                } else {
                    let mut ssh_cmd = tokio::process::Command::new("ssh");
                    ssh_cmd.arg("-p").arg(port.to_string())
                           .arg("-o").arg("StrictHostKeyChecking=no")
                           .arg("-o").arg("UserKnownHostsFile=/dev/null")
                           .arg(format!("{}@{}", username, host))
                           .arg(format!("cat {}", remote_path));
                    ssh_cmd
                };

                let output = cmd.output().await
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

                let mut cmd = if auth_type == "password" {
                    let password = auth_data["password"].as_str().unwrap_or("");
                    let mut sshpass_cmd = tokio::process::Command::new("sshpass");
                    sshpass_cmd.arg("-p").arg(password)
                              .arg("scp")
                              .arg("-P").arg(port.to_string())
                              .arg("-o").arg("StrictHostKeyChecking=no")
                              .arg("-o").arg("UserKnownHostsFile=/dev/null")
                              .arg(temp_file.path())
                              .arg(format!("{}@{}:{}", username, host, remote_path));
                    sshpass_cmd
                } else {
                    let mut scp_cmd = tokio::process::Command::new("scp");
                    scp_cmd.arg("-P").arg(port.to_string())
                           .arg("-o").arg("StrictHostKeyChecking=no")
                           .arg("-o").arg("UserKnownHostsFile=/dev/null")
                           .arg(temp_file.path())
                           .arg(format!("{}@{}:{}", username, host, remote_path));
                    scp_cmd
                };

                let output = cmd.output().await
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
                let mut cmd = if auth_type == "password" {
                    let password = auth_data["password"].as_str().unwrap_or("");
                    let mut sshpass_cmd = tokio::process::Command::new("sshpass");
                    sshpass_cmd.arg("-p").arg(password)
                              .arg("ssh")
                              .arg("-p").arg(port.to_string())
                              .arg("-o").arg("StrictHostKeyChecking=no")
                              .arg("-o").arg("UserKnownHostsFile=/dev/null")
                              .arg(format!("{}@{}", username, host))
                              .arg(format!("rm -f {}", remote_path));
                    sshpass_cmd
                } else {
                    let mut ssh_cmd = tokio::process::Command::new("ssh");
                    ssh_cmd.arg("-p").arg(port.to_string())
                           .arg("-o").arg("StrictHostKeyChecking=no")
                           .arg("-o").arg("UserKnownHostsFile=/dev/null")
                           .arg(format!("{}@{}", username, host))
                           .arg(format!("rm -f {}", remote_path));
                    ssh_cmd
                };

                let output = cmd.output().await
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

    async fn list_objects(&self, base_url: &str, path: &str) -> Result<Vec<String>> {
        // For HTTPS, we need an API endpoint to list objects
        // This is a simplified implementation
        let list_url = format!("{}/list/{}", base_url, path);
        let credentials = crate::Credential::Token { value: "dummy".to_string() };
        let auth = self.authenticate(&credentials).await?;
        
        match self.get(&list_url, &auth).await {
            Ok(data) => {
                let list_str = String::from_utf8_lossy(&data);
                Ok(list_str.lines().map(|s| s.to_string()).collect())
            },
            Err(_) => Ok(Vec::new()) // Return empty list if listing fails
        }
    }

    async fn download_object(&self, base_url: &str, object_path: &str) -> Result<Vec<u8>> {
        let object_url = format!("{}/{}", base_url, object_path);
        let credentials = crate::Credential::Token { value: "dummy".to_string() };
        let auth = self.authenticate(&credentials).await?;
        
        self.get(&object_url, &auth).await
    }

    async fn upload_object(&self, base_url: &str, object_path: &str, data: &[u8]) -> Result<()> {
        let object_url = format!("{}/{}", base_url, object_path);
        let credentials = crate::Credential::Token { value: "dummy".to_string() };
        let auth = self.authenticate(&credentials).await?;
        
        self.put(&object_url, data, &auth).await?;
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
            crate::Credential::Password { username, password } => {
                let token_data = serde_json::json!({
                    "type": "password",
                    "username": username,
                    "password": password
                });
                Ok(AuthToken {
                    token: token_data.to_string(),
                    expires_at: None,
                    refresh_token: None,
                })
            },
            _ => Err(NetError::Protocol {
                message: "SSH requires SSH key or password authentication".to_string(),
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

    async fn list_objects(&self, base_url: &str, path: &str) -> Result<Vec<String>> {
        // Parse SSH URL to get connection details
        let url = url::Url::parse(base_url).map_err(|e| NetError::Protocol {
            message: format!("Invalid SSH URL: {}", e),
        })?;

        let host = url.host_str().ok_or_else(|| NetError::Protocol {
            message: "No host in SSH URL".to_string(),
        })?;
        
        let port = url.port().unwrap_or(22);
        let base_path = url.path();
        let username = url.username();
        
        let remote_path = format!("{}/{}", base_path, path);

        // Check for password in URL for authentication
        let mut cmd = if let Some(password) = url.password() {
            let mut sshpass_cmd = tokio::process::Command::new("sshpass");
            sshpass_cmd.arg("-p").arg(password)
                      .arg("ssh")
                      .arg("-p").arg(port.to_string())
                      .arg("-o").arg("StrictHostKeyChecking=no")
                      .arg("-o").arg("UserKnownHostsFile=/dev/null")
                      .arg(format!("{}@{}", username, host))
                      .arg(format!("ls -1 {}", remote_path));
            sshpass_cmd
        } else {
            let mut ssh_cmd = tokio::process::Command::new("ssh");
            ssh_cmd.arg("-p").arg(port.to_string())
                   .arg("-o").arg("StrictHostKeyChecking=no")
                   .arg("-o").arg("UserKnownHostsFile=/dev/null")
                   .arg(format!("{}@{}", username, host))
                   .arg(format!("ls -1 {}", remote_path));
            ssh_cmd
        };

        let output = cmd.output().await
            .map_err(|e| NetError::Protocol {
                message: format!("SSH list command failed: {}", e),
            })?;

        if output.status.success() {
            let list_str = String::from_utf8_lossy(&output.stdout);
            Ok(list_str.lines().filter(|s| !s.is_empty()).map(|s| s.to_string()).collect())
        } else {
            // Directory might not exist, return empty list
            Ok(Vec::new())
        }
    }

    async fn download_object(&self, base_url: &str, object_path: &str) -> Result<Vec<u8>> {
        // Parse SSH URL to get connection details
        let url = url::Url::parse(base_url).map_err(|e| NetError::Protocol {
            message: format!("Invalid SSH URL: {}", e),
        })?;

        let host = url.host_str().ok_or_else(|| NetError::Protocol {
            message: "No host in SSH URL".to_string(),
        })?;
        
        let port = url.port().unwrap_or(22);
        let base_path = url.path();
        let username = url.username();
        
        let remote_path = format!("{}/{}", base_path, object_path);

        // Check for password in URL for authentication
        let mut cmd = if let Some(password) = url.password() {
            let mut sshpass_cmd = tokio::process::Command::new("sshpass");
            sshpass_cmd.arg("-p").arg(password)
                      .arg("ssh")
                      .arg("-p").arg(port.to_string())
                      .arg("-o").arg("StrictHostKeyChecking=no")
                      .arg("-o").arg("UserKnownHostsFile=/dev/null")
                      .arg(format!("{}@{}", username, host))
                      .arg(format!("cat {}", remote_path));
            sshpass_cmd
        } else {
            let mut ssh_cmd = tokio::process::Command::new("ssh");
            ssh_cmd.arg("-p").arg(port.to_string())
                   .arg("-o").arg("StrictHostKeyChecking=no")
                   .arg("-o").arg("UserKnownHostsFile=/dev/null")
                   .arg(format!("{}@{}", username, host))
                   .arg(format!("cat {}", remote_path));
            ssh_cmd
        };

        let output = cmd.output().await
            .map_err(|e| NetError::Protocol {
                message: format!("SSH download command failed: {}", e),
            })?;

        if output.status.success() {
            Ok(output.stdout)
        } else {
            Err(NetError::Protocol {
                message: format!("SSH download failed: {}", String::from_utf8_lossy(&output.stderr)),
            })
        }
    }

    async fn upload_object(&self, base_url: &str, object_path: &str, data: &[u8]) -> Result<()> {
        // Parse SSH URL to get connection details
        let url = url::Url::parse(base_url).map_err(|e| NetError::Protocol {
            message: format!("Invalid SSH URL: {}", e),
        })?;

        let host = url.host_str().ok_or_else(|| NetError::Protocol {
            message: "No host in SSH URL".to_string(),
        })?;
        
        let port = url.port().unwrap_or(22);
        let base_path = url.path();
        let username = url.username();
        
        let remote_path = format!("{}/{}", base_path, object_path);

        // Create remote directory structure first
        let remote_dir = std::path::Path::new(&remote_path).parent().unwrap().to_string_lossy();
        
        let mut mkdir_cmd = if let Some(password) = url.password() {
            let mut sshpass_cmd = tokio::process::Command::new("sshpass");
            sshpass_cmd.arg("-p").arg(password)
                      .arg("ssh")
                      .arg("-p").arg(port.to_string())
                      .arg("-o").arg("StrictHostKeyChecking=no")
                      .arg("-o").arg("UserKnownHostsFile=/dev/null")
                      .arg(format!("{}@{}", username, host))
                      .arg(format!("mkdir -p {}", remote_dir));
            sshpass_cmd
        } else {
            let mut ssh_cmd = tokio::process::Command::new("ssh");
            ssh_cmd.arg("-p").arg(port.to_string())
                   .arg("-o").arg("StrictHostKeyChecking=no")
                   .arg("-o").arg("UserKnownHostsFile=/dev/null")
                   .arg(format!("{}@{}", username, host))
                   .arg(format!("mkdir -p {}", remote_dir));
            ssh_cmd
        };
        
        let _mkdir_output = mkdir_cmd.output().await;

        // Use SCP to upload the file
        let temp_file = tempfile::NamedTempFile::new()
            .map_err(|e| NetError::Protocol {
                message: format!("Failed to create temp file: {}", e),
            })?;

        tokio::fs::write(temp_file.path(), data).await
            .map_err(|e| NetError::Protocol {
                message: format!("Failed to write temp file: {}", e),
            })?;

        let mut cmd = if let Some(password) = url.password() {
            let mut sshpass_cmd = tokio::process::Command::new("sshpass");
            sshpass_cmd.arg("-p").arg(password)
                      .arg("scp")
                      .arg("-P").arg(port.to_string())
                      .arg("-o").arg("StrictHostKeyChecking=no")
                      .arg("-o").arg("UserKnownHostsFile=/dev/null")
                      .arg(temp_file.path())
                      .arg(format!("{}@{}:{}", username, host, remote_path));
            sshpass_cmd
        } else {
            let mut scp_cmd = tokio::process::Command::new("scp");
            scp_cmd.arg("-P").arg(port.to_string())
                   .arg("-o").arg("StrictHostKeyChecking=no")
                   .arg("-o").arg("UserKnownHostsFile=/dev/null")
                   .arg(temp_file.path())
                   .arg(format!("{}@{}:{}", username, host, remote_path));
            scp_cmd
        };

        let output = cmd.output().await
            .map_err(|e| NetError::Protocol {
                message: format!("SCP upload command failed: {}", e),
            })?;

        if output.status.success() {
            Ok(())
        } else {
            Err(NetError::Protocol {
                message: format!("SSH upload failed: {}", String::from_utf8_lossy(&output.stderr)),
            })
        }
    }
}