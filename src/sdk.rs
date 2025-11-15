//! Playdate SDK detection and interaction

use crate::error::{CrankError, Result};
use colored::Colorize;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Represents the Playdate SDK installation
#[derive(Debug, Clone)]
pub struct Sdk {
    /// Root path to the SDK
    pub path: PathBuf,
}

impl Sdk {
    /// Detect and locate the Playdate SDK
    pub fn detect() -> Result<Self> {
        // Try multiple detection methods in order
        if let Some(path) = Self::from_env() {
            return Ok(Self { path });
        }

        if let Some(path) = Self::from_default_location() {
            return Ok(Self { path });
        }

        Err(CrankError::SdkNotFound)
    }

    /// Try to get SDK path from PLAYDATE_SDK_PATH environment variable
    fn from_env() -> Option<PathBuf> {
        env::var("PLAYDATE_SDK_PATH")
            .ok()
            .map(PathBuf::from)
            .filter(|p| p.exists() && p.is_dir())
    }

    /// Try platform-specific default SDK locations
    fn from_default_location() -> Option<PathBuf> {
        let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).ok()?;
        let home_path = PathBuf::from(home);

        let candidates = if cfg!(target_os = "macos") {
            vec![
                home_path.join("Developer").join("PlaydateSDK"),
                PathBuf::from("/usr/local/share/PlaydateSDK"),
            ]
        } else if cfg!(target_os = "windows") {
            vec![
                home_path.join("Documents").join("PlaydateSDK"),
                home_path.join("PlaydateSDK"),
            ]
        } else {
            // Linux and other Unix-like systems
            vec![
                home_path.join("PlaydateSDK"),
                PathBuf::from("/opt/PlaydateSDK"),
                PathBuf::from("/usr/local/PlaydateSDK"),
            ]
        };

        candidates.into_iter().find(|p| p.exists() && p.is_dir())
    }

    /// Get the path to the pdc compiler
    pub fn pdc_path(&self) -> Result<PathBuf> {
        let pdc = if cfg!(target_os = "windows") {
            self.path.join("bin").join("pdc.exe")
        } else {
            self.path.join("bin").join("pdc")
        };

        if pdc.exists() {
            Ok(pdc)
        } else {
            Err(CrankError::SdkComponentNotFound {
                component: "pdc".to_string(),
            })
        }
    }

    /// Get the path to the Playdate Simulator
    pub fn simulator_path(&self) -> Result<PathBuf> {
        let simulator = if cfg!(target_os = "macos") {
            // macOS: simulator is in an app bundle
            self.path
                .join("bin")
                .join("Playdate Simulator.app")
                .join("Contents")
                .join("MacOS")
                .join("Playdate Simulator")
        } else if cfg!(target_os = "windows") {
            self.path.join("bin").join("PlaydateSimulator.exe")
        } else {
            // Linux
            self.path.join("bin").join("PlaydateSimulator")
        };

        if simulator.exists() || (cfg!(target_os = "macos") && simulator.parent().unwrap().exists())
        {
            Ok(simulator)
        } else {
            Err(CrankError::SdkComponentNotFound {
                component: "PlaydateSimulator".to_string(),
            })
        }
    }

    /// Compile a Playdate project using pdc
    pub fn compile<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        source_dir: P,
        output_path: Q,
        verbose: bool,
    ) -> Result<()> {
        let pdc = self.pdc_path()?;
        let source = source_dir.as_ref();
        let output = output_path.as_ref();

        if !source.exists() {
            return Err(CrankError::InvalidProject(format!(
                "Source directory not found: {}",
                source.display()
            )));
        }

        // Create output directory if it doesn't exist
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut cmd = Command::new(&pdc);
        cmd.arg(source).arg(output);

        if verbose {
            // In verbose mode, show all pdc output
            cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

            let status = cmd.status()?;

            if !status.success() {
                return Err(CrankError::BuildFailed(
                    "Compilation failed. See output above for details.".to_string(),
                ));
            }
        } else {
            // Capture output to check for errors/warnings
            cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

            let output_result = cmd.output()?;

            // Check for warnings in stdout/stderr
            let stdout = String::from_utf8_lossy(&output_result.stdout);
            let stderr = String::from_utf8_lossy(&output_result.stderr);

            // Show warnings if any (pdc outputs to stderr even for warnings)
            if !stderr.is_empty() && stderr.contains("warning") {
                eprintln!("{} Build warnings:", "⚠".yellow().bold());
                for line in stderr.lines() {
                    if !line.trim().is_empty() {
                        eprintln!("  {}", line);
                    }
                }
            }

            if !output_result.status.success() {
                let error_msg = if !stderr.is_empty() {
                    stderr.trim().to_string()
                } else if !stdout.is_empty() {
                    stdout.trim().to_string()
                } else {
                    "Compilation failed with unknown error".to_string()
                };

                return Err(CrankError::BuildFailed(error_msg));
            }
        }

        Ok(())
    }

    /// Launch the Playdate Simulator with a .pdx file
    pub fn launch_simulator<P: AsRef<Path>>(&self, pdx_path: P) -> Result<()> {
        let simulator = self.simulator_path()?;
        let pdx = pdx_path.as_ref();

        // Verify PDX bundle exists
        if !pdx.exists() {
            return Err(CrankError::InvalidProject(format!(
                "PDX bundle not found: {}",
                pdx.display()
            )));
        }

        // Verify it's a directory (PDX bundles are directories)
        if !pdx.is_dir() {
            return Err(CrankError::InvalidProject(format!(
                "Invalid PDX bundle (not a directory): {}",
                pdx.display()
            )));
        }

        let mut cmd = if cfg!(target_os = "macos") {
            // On macOS, use 'open' command to launch the app bundle
            // This properly handles the app bundle structure
            let mut c = Command::new("open");
            c.arg("-a");

            // Use the app bundle path, not the executable inside
            let app_bundle = self.path.join("bin").join("Playdate Simulator.app");

            c.arg(&app_bundle);
            c.arg(pdx);
            c
        } else {
            // On Windows and Linux, run the simulator directly
            let mut c = Command::new(&simulator);
            c.arg(pdx);
            c
        };

        // Detach from parent process so simulator continues after pdbm exits
        cmd.stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        cmd.spawn().map_err(|e| {
            CrankError::SimulatorFailed(format!(
                "Failed to launch simulator: {}. \n\
                     Simulator path: {}\n\
                     Try running it manually to verify it works.",
                e,
                simulator.display()
            ))
        })?;

        Ok(())
    }

    /// Check if the SDK path is valid
    pub fn validate(&self) -> Result<()> {
        if !self.path.exists() || !self.path.is_dir() {
            return Err(CrankError::SdkNotFound);
        }

        // Check for essential components
        self.pdc_path()?;
        self.simulator_path()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdk_detection() {
        // This test will pass if SDK is installed, skip otherwise
        if let Ok(sdk) = Sdk::detect() {
            assert!(sdk.path.exists());
            assert!(sdk.pdc_path().is_ok());
        }
    }
}
