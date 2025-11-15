//! Build Playdate projects

use crate::error::Result;
use crate::project::Project;
use crate::sdk::Sdk;
use colored::Colorize;
use std::time::Instant;

/// Execute the 'build' command
pub fn execute(release: bool, verbose: bool) -> Result<()> {
    let start = Instant::now();

    let mode = if release { "release" } else { "debug" };
    println!(
        "{} Playdate project ({} mode)...",
        "Building".green().bold(),
        mode.cyan()
    );
    println!();

    // Load project
    let project = Project::find_and_load()?;

    if verbose {
        println!("  {} {}", "Project:".dimmed(), project.config.package.name);
        println!(
            "  {} {}",
            "Version:".dimmed(),
            project.config.package.version
        );
        println!(
            "  {} {}",
            "Language:".dimmed(),
            project.config.build.language
        );
        println!();
    }

    // Validate project structure
    if verbose {
        println!("  {} Validating project structure...", "→".cyan());
    }
    project.validate()?;

    // Detect SDK
    if verbose {
        println!("  {} Detecting Playdate SDK...", "→".cyan());
    }
    let sdk = Sdk::detect()?;

    if verbose {
        println!("    {} {}", "SDK:".dimmed(), sdk.path.display());
        println!();
    }

    // Ensure output directory exists
    project.ensure_output_dir()?;

    // Generate pdxinfo from configuration
    if verbose {
        println!("  {} Generating pdxinfo...", "→".cyan());
    }
    project.generate_pdxinfo()?;

    // Compile project
    let source_dir = project.source_dir();
    let pdx_path = project.pdx_path();

    if verbose {
        println!("  {} Compiling...", "→".cyan());
        println!("    {} {}", "Source:".dimmed(), source_dir.display());
        println!("    {} {}", "Output:".dimmed(), pdx_path.display());
        println!();
    }

    sdk.compile(&source_dir, &pdx_path, verbose)?;

    let duration = start.elapsed();

    println!();
    println!(
        "{} Built successfully in {:.2}s",
        "✓".green().bold(),
        duration.as_secs_f64()
    );
    println!();
    println!("  {} {}", "Output:".bold(), pdx_path.display());

    // Calculate directory size
    if let Ok(size) = calculate_dir_size(&pdx_path) {
        let size_kb = size / 1024;
        let size_mb = size_kb as f64 / 1024.0;

        if size_mb >= 1.0 {
            println!("  {} {:.2} MB", "Size:".bold(), size_mb);
        } else {
            println!("  {} {} KB", "Size:".bold(), size_kb);
        }
    }

    // Show next steps
    if !verbose {
        println!();
        println!("{}", "Next steps:".dimmed());
        println!("  {} {}", "crank".cyan(), "run".yellow());
    }

    Ok(())
}

/// Calculate the total size of a directory recursively
fn calculate_dir_size(path: &std::path::Path) -> std::io::Result<u64> {
    let mut total = 0;

    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                total += calculate_dir_size(&entry_path)?;
            } else {
                total += entry.metadata()?.len();
            }
        }
    } else {
        total = std::fs::metadata(path)?.len();
    }

    Ok(total)
}
