//! crank
//!
//! A command-line build tool for Playdate game development, inspired by Cargo.
//! Provides project scaffolding, build automation, and development tools.

pub mod cli;
pub mod commands;
pub mod config;
pub mod error;
pub mod project;
pub mod sdk;

// Re-export commonly used types
pub use config::PlaydateConfig;
pub use error::{CrankError, Result};
pub use project::Project;
pub use sdk::Sdk;
