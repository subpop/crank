//! Configuration file parsing and management

use crate::error::{CrankError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Root configuration structure for Playdate.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaydateConfig {
    pub package: PackageConfig,
    #[serde(default)]
    pub build: BuildConfig,
    #[serde(default)]
    pub playdate: PlaydateMetadata,
    #[serde(default)]
    pub dev: DevConfig,
    #[serde(default)]
    pub dependencies: Dependencies,
}

/// Package metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageConfig {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub description: String,
    pub bundle_id: String,
}

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_source_dir")]
    pub source_dir: String,
    #[serde(default = "default_output_dir")]
    pub output_dir: String,
    #[serde(default = "default_assets_dir")]
    pub assets_dir: String,
}

/// Playdate-specific metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaydateMetadata {
    #[serde(default)]
    pub content_warning: String,
    #[serde(default)]
    pub content_warning_2: String,
    #[serde(default = "default_image_path")]
    pub image_path: String,
    #[serde(default = "default_build_number")]
    pub build_number: u32,
}

/// Development configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevConfig {
    #[serde(default = "default_true")]
    pub hot_reload: bool,
    #[serde(default = "default_true")]
    pub auto_build: bool,
    #[serde(default)]
    pub simulator_path: Option<String>,
}

/// Dependencies configuration (for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Dependencies {
    #[serde(flatten)]
    pub deps: std::collections::HashMap<String, String>,
}

// Default value functions
fn default_language() -> String {
    "lua".to_string()
}

fn default_source_dir() -> String {
    "source".to_string()
}

fn default_output_dir() -> String {
    "build".to_string()
}

fn default_assets_dir() -> String {
    "assets".to_string()
}

fn default_image_path() -> String {
    "icon".to_string()
}

fn default_build_number() -> u32 {
    1
}

fn default_true() -> bool {
    true
}

// Implement Default for configs
impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            language: default_language(),
            source_dir: default_source_dir(),
            output_dir: default_output_dir(),
            assets_dir: default_assets_dir(),
        }
    }
}

impl Default for PlaydateMetadata {
    fn default() -> Self {
        Self {
            content_warning: String::new(),
            content_warning_2: String::new(),
            image_path: default_image_path(),
            build_number: default_build_number(),
        }
    }
}

impl Default for DevConfig {
    fn default() -> Self {
        Self {
            hot_reload: true,
            auto_build: true,
            simulator_path: None,
        }
    }
}

impl PlaydateConfig {
    /// Load configuration from a Playdate.toml file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref()).map_err(|_| CrankError::ConfigNotFound)?;

        toml::from_str(&content).map_err(|e| CrankError::ConfigParseError(e.to_string()))
    }

    /// Find and load the nearest Playdate.toml file by searching up the directory tree
    pub fn find_and_load() -> Result<(Self, PathBuf)> {
        let current_dir = std::env::current_dir()?;
        let mut dir = current_dir.as_path();

        loop {
            let config_path = dir.join("Playdate.toml");
            if config_path.exists() {
                let config = Self::from_file(&config_path)?;
                return Ok((config, dir.to_path_buf()));
            }

            match dir.parent() {
                Some(parent) => dir = parent,
                None => return Err(CrankError::ConfigNotFound),
            }
        }
    }

    /// Save configuration to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| CrankError::Other(format!("Failed to serialize config: {}", e)))?;

        fs::write(path, content)?;
        Ok(())
    }

    /// Generate a default configuration for a new project
    pub fn new_project(name: &str, bundle_id: &str) -> Self {
        Self {
            package: PackageConfig {
                name: name.to_string(),
                version: "1.0.0".to_string(),
                author: String::new(),
                description: String::new(),
                bundle_id: bundle_id.to_string(),
            },
            build: BuildConfig::default(),
            playdate: PlaydateMetadata::default(),
            dev: DevConfig::default(),
            dependencies: Dependencies::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let toml_str = r#"
            [package]
            name = "test-game"
            version = "1.0.0"
            bundle_id = "com.example.testgame"

            [build]
            language = "lua"
        "#;

        let config: PlaydateConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.package.name, "test-game");
        assert_eq!(config.build.language, "lua");
    }
}
