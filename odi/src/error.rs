//! Error types for ODI CLI

use thiserror::Error;

/// Result type for CLI operations
pub type Result<T> = std::result::Result<T, OdiError>;

/// Error type for CLI operations
#[derive(Debug, Error)]
pub enum OdiError {
    #[error("Core error: {0}")]
    Core(#[from] odi_core::CoreError),

    #[error("Filesystem error: {0}")]
    Filesystem(#[from] odi_fs::FsError),

    #[error("Network error: {0}")]
    Network(#[from] odi_net::NetError),

    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Storage error: {message}")]
    Storage { message: String },

    #[error("Synchronization error: {message}")]
    Sync { message: String },

    #[error("Not initialized: {message}")]
    NotInitialized { message: String },

    #[error("Command error: {message}")]
    Command { message: String },

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("IO error: {message}")]
    Io { message: String },

    #[error("Team not found: {0}")]
    TeamNotFound(String),

    #[error("User not found: {0}")]
    UserNotFound(String),
}

// Conversion from std::io::Error
impl From<std::io::Error> for OdiError {
    fn from(error: std::io::Error) -> Self {
        OdiError::Io {
            message: error.to_string(),
        }
    }
}

/// T076: User-friendly error message formatting
impl OdiError {
    /// Convert technical errors to user-friendly messages
    pub fn format_user_friendly(&self) -> String {
        match self {
            OdiError::Core(core_error) => {
                format!("🔧 Core Error\n{}\n\n💡 Tip: This is an internal ODI error", core_error)
            },
            OdiError::Filesystem(fs_error) => {
                format!("💾 Filesystem Error\n{}\n\n💡 Tip: Check file permissions and disk space", fs_error)
            },
            OdiError::Network(net_error) => {
                format!("🌐 Network Error\n{}\n\n💡 Tip: Check your internet connection and remote URLs", net_error)
            },
            OdiError::NotInitialized { message } => {
                format!("❌ Workspace Error\n{}\n\n💡 Tip: Use 'odi init' to set up a new workspace", message)
            },
            OdiError::Config { message } => {
                format!("⚙️  Configuration Error\n{}\n\n💡 Tip: Check your .odi/config file or run 'odi config --help'", message)
            },
            OdiError::Storage { message } => {
                format!("💾 Storage Error\n{}\n\n💡 Tip: Check file permissions and disk space", message)
            },
            OdiError::Sync { message } => {
                format!("🔄 Synchronization Error\n{}\n\n💡 Tip: Try 'odi remote status' to check remote connectivity", message)
            },
            OdiError::Command { message } => {
                format!("⚠️  Command Error\n{}\n\n💡 Tip: Use 'odi --help' for command usage", message)
            },
            OdiError::Validation { message } => {
                format!("❌ Validation Error\n{}\n\n💡 Tip: Check your input and try again", message)
            },
            OdiError::Io { message } => {
                format!("📁 File System Error\n{}\n\n💡 Tip: Check file permissions and paths", message)
            },
            OdiError::TeamNotFound(team_name) => {
                format!("❌ Team Not Found\nTeam '{}' does not exist\n\n💡 Tip: Use 'odi team list' to see available teams", team_name)
            },
            OdiError::UserNotFound(user_name) => {
                format!("❌ User Not Found\nUser '{}' does not exist\n\n💡 Tip: Check the username or create the user first", user_name)
            },
        }
    }
}