//! Error types for ODI Network

use thiserror::Error;

/// Result type for network operations
pub type Result<T> = std::result::Result<T, NetError>;

/// Error type for network operations
#[derive(Debug, Error)]
pub enum NetError {
    #[error("Connection failed: {url}")]
    ConnectionFailed { url: String },

    #[error("Authentication failed: {method}")]
    AuthenticationFailed { method: String },

    #[error("Protocol error: {message}")]
    ProtocolError { message: String },

    #[error("Timeout: operation took longer than {seconds}s")]
    Timeout { seconds: u64 },

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("TLS error: {message}")]
    TlsError { message: String },

    #[error("Core error: {0}")]
    Core(#[from] odi_core::CoreError),
}