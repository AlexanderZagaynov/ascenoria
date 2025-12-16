//! Error types for data loading.
//!
//! Defines all error variants that can occur while reading,
//! parsing, or validating TOML game data files.

use thiserror::Error;

/// Errors that can occur while loading TOML game data.
#[derive(Debug, Error)]
pub enum DataLoadError {
    /// File read failure.
    #[error("Failed to read {path}: {source}")]
    Io {
        /// Source I/O error.
        source: std::io::Error,
        /// Path that failed.
        path: String,
    },
    /// RON parse failure.
    #[error("Failed to parse {path}: {source}")]
    Parse {
        /// RON parse error.
        source: ron::error::SpannedError,
        /// Path that failed.
        path: String,
    },
    /// Schema version is newer than the loader understands.
    #[error("Unsupported schema version {found} in {path}; current version is {current}")]
    UnsupportedSchemaVersion {
        /// Version found in manifest.
        found: u32,
        /// Latest version supported by the loader.
        current: u32,
        /// File path that declared the version.
        path: String,
    },
    /// Duplicate identifier encountered.
    #[error("Duplicate {kind} id encountered: {id}")]
    DuplicateId {
        /// Entity type string.
        kind: &'static str,
        /// Duplicated identifier.
        id: String,
    },
    /// Validation failure.
    #[error("Validation failed for {kind} '{id}': {message}")]
    Validation {
        /// Entity type string.
        kind: &'static str,
        /// Identifier that failed validation.
        id: String,
        /// Validation error details.
        message: String,
    },
}
