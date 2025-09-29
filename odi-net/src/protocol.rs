use serde::{Deserialize, Serialize};
use crate::{Result, NetError, AuthToken};

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
}

#[async_trait::async_trait]
impl ProtocolHandler for HttpsHandler {
    async fn authenticate(&self, _credentials: &crate::Credential) -> Result<AuthToken> {
        // Mock implementation for contract tests
        Err(NetError::Protocol {
            message: "ProtocolHandler::authenticate not implemented yet".to_string(),
        })
    }

    async fn get(&self, _path: &str, _auth: &AuthToken) -> Result<Vec<u8>> {
        Err(NetError::Protocol {
            message: "ProtocolHandler::get not implemented yet".to_string(),
        })
    }

    async fn post(&self, _path: &str, _data: &[u8], _auth: &AuthToken) -> Result<Vec<u8>> {
        Err(NetError::Protocol {
            message: "ProtocolHandler::post not implemented yet".to_string(),
        })
    }

    async fn put(&self, _path: &str, _data: &[u8], _auth: &AuthToken) -> Result<Vec<u8>> {
        Err(NetError::Protocol {
            message: "ProtocolHandler::put not implemented yet".to_string(),
        })
    }

    async fn delete(&self, _path: &str, _auth: &AuthToken) -> Result<()> {
        Err(NetError::Protocol {
            message: "ProtocolHandler::delete not implemented yet".to_string(),
        })
    }
}

#[async_trait::async_trait]
impl ProtocolHandler for SshHandler {
    async fn authenticate(&self, _credentials: &crate::Credential) -> Result<AuthToken> {
        Err(NetError::Protocol {
            message: "ProtocolHandler::authenticate not implemented yet".to_string(),
        })
    }

    async fn get(&self, _path: &str, _auth: &AuthToken) -> Result<Vec<u8>> {
        Err(NetError::Protocol {
            message: "ProtocolHandler::get not implemented yet".to_string(),
        })
    }

    async fn post(&self, _path: &str, _data: &[u8], _auth: &AuthToken) -> Result<Vec<u8>> {
        Err(NetError::Protocol {
            message: "ProtocolHandler::post not implemented yet".to_string(),
        })
    }

    async fn put(&self, _path: &str, _data: &[u8], _auth: &AuthToken) -> Result<Vec<u8>> {
        Err(NetError::Protocol {
            message: "ProtocolHandler::put not implemented yet".to_string(),
        })
    }

    async fn delete(&self, _path: &str, _auth: &AuthToken) -> Result<()> {
        Err(NetError::Protocol {
            message: "ProtocolHandler::delete not implemented yet".to_string(),
        })
    }
}