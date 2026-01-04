//! Error types for the Zotero client library.

use thiserror::Error;

/// Errors that can occur when using the Zotero client.
#[derive(Error, Debug)]
pub enum ZoteroClientError {
    /// Connection error when the Zotero server is unreachable.
    #[error("connection error: {0}")]
    Connection(#[from] reqwest::Error),

    /// API returned an error response.
    #[error("API error (status {status}): {message}")]
    Api {
        /// HTTP status code.
        status: u16,
        /// Error message from the API.
        message: String,
    },

    /// Failed to parse the API response.
    #[error("parse error: {0}")]
    Parse(#[from] serde_json::Error),

    /// The requested item was not found.
    #[error("item not found: {key}")]
    NotFound {
        /// The key that was not found.
        key: String,
    },

    /// PDF extraction error (only available with `pdf` feature).
    #[cfg(feature = "pdf")]
    #[error("PDF error: {0}")]
    Pdf(String),

    /// Invalid input provided.
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

/// Result type alias for Zotero client operations.
pub type Result<T> = std::result::Result<T, ZoteroClientError>;
