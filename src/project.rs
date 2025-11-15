//! Project structure and management

use crate::config::PlaydateConfig;
use crate::error::{CrankError, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a Playdate project
#[derive(Debug)]
pub struct Project {
    /// Root directory of the project
    pub root: PathBuf,
    /// Project configuration
    pub config: PlaydateConfig,
}

impl Project {
    /// Load a project from the current directory or parent directories
    pub fn find_and_load() -> Result<Self> {
        let (config, root) = PlaydateConfig::find_and_load()?;
        Ok(Self { root, config })
    }

    /// Load a project from a specific directory
    pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Self> {
        let root = path.as_ref().to_path_buf();
        let config_path = root.join("Playdate.toml");
        let config = PlaydateConfig::from_file(config_path)?;
        Ok(Self { root, config })
    }

    /// Get the source directory path
    pub fn source_dir(&self) -> PathBuf {
        self.root.join(&self.config.build.source_dir)
    }

    /// Get the output directory path
    pub fn output_dir(&self) -> PathBuf {
        self.root.join(&self.config.build.output_dir)
    }

    /// Get the assets directory path
    pub fn assets_dir(&self) -> PathBuf {
        self.root.join(&self.config.build.assets_dir)
    }

    /// Get the tests directory path
    pub fn tests_dir(&self) -> PathBuf {
        self.root.join("tests")
    }

    /// Get the output .pdx path
    pub fn pdx_path(&self) -> PathBuf {
        self.output_dir()
            .join(format!("{}.pdx", self.config.package.name))
    }

    /// Validate the project structure
    pub fn validate(&self) -> Result<()> {
        // Check if source directory exists
        let source_dir = self.source_dir();
        if !source_dir.exists() {
            return Err(CrankError::InvalidProject(format!(
                "Source directory not found: {}",
                source_dir.display()
            )));
        }

        // Check for entry point based on language
        let entry_point = match self.config.build.language.as_str() {
            "lua" => source_dir.join("main.lua"),
            "swift" => source_dir.join("main.swift"),
            _ => {
                return Err(CrankError::InvalidProject(format!(
                    "Unsupported language: {}",
                    self.config.build.language
                )))
            }
        };

        if !entry_point.exists() {
            return Err(CrankError::InvalidProject(format!(
                "Entry point not found: {}",
                entry_point.display()
            )));
        }

        Ok(())
    }

    /// Create the build output directory if it doesn't exist
    pub fn ensure_output_dir(&self) -> Result<()> {
        let output_dir = self.output_dir();
        if !output_dir.exists() {
            fs::create_dir_all(&output_dir)?;
        }
        Ok(())
    }

    /// Clean build artifacts
    pub fn clean(&self) -> Result<()> {
        let output_dir = self.output_dir();
        if output_dir.exists() {
            fs::remove_dir_all(&output_dir)?;
        }
        Ok(())
    }

    /// Generate pdxinfo file from configuration
    pub fn generate_pdxinfo(&self) -> Result<()> {
        let pdxinfo_content = format!(
            "name={}\nauthor={}\ndescription={}\nbundleID={}\nversion={}\nbuildNumber={}\nimagePath={}\n",
            self.config.package.name,
            self.config.package.author,
            self.config.package.description,
            self.config.package.bundle_id,
            self.config.package.version,
            self.config.playdate.build_number,
            self.config.playdate.image_path
        );

        let pdxinfo_path = self.source_dir().join("pdxinfo");
        fs::write(pdxinfo_path, pdxinfo_content)?;
        Ok(())
    }
}

/// Create a new Playdate project
pub fn create_new_project<P: AsRef<Path>>(name: &str, path: P, template: &str) -> Result<Project> {
    let project_path = path.as_ref().join(name);

    // Validate project name
    if !is_valid_project_name(name) {
        return Err(CrankError::InvalidProjectName(name.to_string()));
    }

    // Check if directory already exists
    if project_path.exists() {
        return Err(CrankError::ProjectExists(project_path));
    }

    // Create project directory structure
    fs::create_dir_all(&project_path)?;
    fs::create_dir_all(project_path.join("source"))?;
    fs::create_dir_all(project_path.join("assets/images"))?;
    fs::create_dir_all(project_path.join("assets/sounds"))?;
    fs::create_dir_all(project_path.join("assets/fonts"))?;
    fs::create_dir_all(project_path.join("tests"))?;

    // Create .gitkeep files in empty asset directories
    fs::write(project_path.join("assets/images/.gitkeep"), "")?;
    fs::write(project_path.join("assets/sounds/.gitkeep"), "")?;
    fs::write(project_path.join("assets/fonts/.gitkeep"), "")?;

    // Generate bundle ID from project name
    let bundle_id = format!("com.example.{}", name.replace("-", ""));

    // Create and save configuration
    let config = PlaydateConfig::new_project(name, &bundle_id);
    config.save(project_path.join("Playdate.toml"))?;

    // Create source files based on template
    match template {
        "lua-basic" => create_lua_basic_template(&project_path, &config)?,
        _ => return Err(CrankError::TemplateNotFound(template.to_string())),
    }

    // Create the project instance
    let project = Project {
        root: project_path,
        config,
    };

    Ok(project)
}

/// Validate project name (alphanumeric and hyphens only)
fn is_valid_project_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        && !name.starts_with('-')
        && !name.ends_with('-')
}

// Embed template files at compile time
const TEMPLATE_MAIN_LUA: &str = include_str!("../templates/lua-basic/main.lua");
const TEMPLATE_TEST_LUA: &str = include_str!("../templates/lua-basic/test_basic.lua");
const TEMPLATE_LUAUNIT: &str = include_str!("../templates/lua-basic/luaunit.lua");
const TEMPLATE_README: &str = include_str!("../templates/lua-basic/README.md");
const TEMPLATE_LUARC: &str = include_str!("../templates/lua-basic/.luarc.json");
const TEMPLATE_GITIGNORE: &str = include_str!("../templates/lua-basic/.gitignore");

/// Create Lua basic template files
fn create_lua_basic_template(project_path: &Path, _config: &PlaydateConfig) -> Result<()> {
    let project_name = project_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Playdate Game");

    // Create main.lua
    fs::write(project_path.join("source/main.lua"), TEMPLATE_MAIN_LUA)?;

    // Create test files
    fs::write(project_path.join("tests/test_basic.lua"), TEMPLATE_TEST_LUA)?;
    fs::write(project_path.join("tests/luaunit.lua"), TEMPLATE_LUAUNIT)?;

    // Create README with project name substitution
    let readme = TEMPLATE_README.replace("{{PROJECT_NAME}}", project_name);
    fs::write(project_path.join("README.md"), readme)?;

    // Create .luarc.json for Lua Language Server support
    fs::write(project_path.join(".luarc.json"), TEMPLATE_LUARC)?;

    // Create .gitignore
    fs::write(project_path.join(".gitignore"), TEMPLATE_GITIGNORE)?;

    // Clone playdate-luacats for type definitions (optional, non-fatal if it fails)
    clone_playdate_luacats(project_path);

    Ok(())
}

/// Clone playdate-luacats repository for Lua Language Server type definitions
fn clone_playdate_luacats(project_path: &Path) {
    use std::process::Command;

    let luacats_path = project_path.join("playdate-luacats");

    // Check if git is available
    let git_check = Command::new("git").arg("--version").output();

    if git_check.is_err() {
        return; // Git not available, skip silently
    }

    // Clone the repository
    let clone_result = Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg("--quiet")
        .arg("https://github.com/notpeter/playdate-luacats.git")
        .arg(&luacats_path)
        .output();

    // If clone succeeds, remove .git directory to avoid nested git repos
    if clone_result.is_ok() {
        let git_dir = luacats_path.join(".git");
        let _ = std::fs::remove_dir_all(git_dir);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_project_names() {
        assert!(is_valid_project_name("my-game"));
        assert!(is_valid_project_name("game123"));
        assert!(is_valid_project_name("my_game"));
        assert!(!is_valid_project_name("-invalid"));
        assert!(!is_valid_project_name("invalid-"));
        assert!(!is_valid_project_name(""));
    }
}
