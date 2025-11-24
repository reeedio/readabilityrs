//! Error types for the readability library.

use thiserror::Error;

/// Result type alias for readability operations
pub type Result<T> = std::result::Result<T, ReadabilityError>;

/// Errors that can occur during readability parsing
#[derive(Error, Debug)]
pub enum ReadabilityError {
    /// Failed to parse HTML document
    #[error("Failed to parse HTML: {0}")]
    ParseError(String),

    /// Invalid URL provided
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// Document structure is invalid or missing required elements
    #[error("Invalid document: {0}")]
    InvalidDocument(String),

    /// JSON-LD parsing error
    #[error("JSON-LD parsing error: {0}")]
    JsonLdError(String),

    /// Maximum element limit exceeded
    #[error("Maximum element limit exceeded: {0}")]
    MaxElementsExceeded(usize),

    /// General error
    #[error("Readability error: {0}")]
    Other(String),
}
