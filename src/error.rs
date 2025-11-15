//! Error types for crank

use std::path::PathBuf;
use thiserror::Error;

/// Custom Result type for crank operations
pub type Result<T> = std::result::Result<T, CrankError>;

/// Errors that can occur during crank operations
#[derive(Error, Debug)]
pub enum CrankError {
    /// Playdate SDK could not be found
    #[error("Playdate SDK not found. Please install the SDK and set PLAYDATE_SDK_PATH environment variable.")]
    SdkNotFound,

    /// SDK component (pdc or simulator) not found
    #[error("SDK component not found: {component}")]
    SdkComponentNotFound { component: String },

    /// Invalid or corrupted project structure
    #[error("Invalid project: {0}")]
    InvalidProject(String),

    /// Project configuration file not found
    #[error("Playdate.toml not found. Are you in a crank project directory?")]
    ConfigNotFound,

    /// Error parsing configuration file
    #[error("Failed to parse Playdate.toml: {0}")]
    ConfigParseError(String),

    /// Build process failed
    #[error("Build failed: {0}")]
    BuildFailed(String),

    /// Simulator failed to launch
    #[error("Failed to launch simulator: {0}")]
    SimulatorFailed(String),

    /// Invalid project name
    #[error("Invalid project name: {0}. Project names must contain only alphanumeric characters and hyphens.")]
    InvalidProjectName(String),

    /// Project already exists
    #[error("Project directory already exists: {0}")]
    ProjectExists(PathBuf),

    /// Template not found
    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    /// File system operation failed
    #[error("File system error: {0}")]
    IoError(#[from] std::io::Error),

    /// TOML deserialization error
    #[error("TOML error: {0}")]
    TomlError(#[from] toml::de::Error),

    /// Watch mode error
    #[error("Watch error: {0}")]
    WatchError(#[from] notify::Error),

    /// Tests failed
    #[error("Some tests failed")]
    TestsFailed,

    /// Generic error with context
    #[error("{0}")]
    Other(String),
}

impl From<&str> for CrankError {
    fn from(s: &str) -> Self {
        CrankError::Other(s.to_string())
    }
}

impl From<String> for CrankError {
    fn from(s: String) -> Self {
        CrankError::Other(s)
    }
}
