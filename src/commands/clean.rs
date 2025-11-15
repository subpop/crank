//! Clean build artifacts

use crate::error::Result;
use crate::project::Project;
use colored::Colorize;

/// Execute the 'clean' command
pub fn execute() -> Result<()> {
    println!("{} build artifacts...", "Cleaning".yellow().bold());

    let project = Project::find_and_load()?;
    let output_dir = project.output_dir();

    if !output_dir.exists() {
        println!("{} Nothing to clean", "✓".green().bold());
        return Ok(());
    }

    project.clean()?;

    println!(
        "{} Cleaned build directory: {}",
        "✓".green().bold(),
        output_dir.display()
    );

    Ok(())
}
