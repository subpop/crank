//! Build and run Playdate projects in the simulator

use crate::error::Result;
use crate::project::Project;
use crate::sdk::Sdk;
use colored::Colorize;

/// Execute the 'run' command
pub fn execute(release: bool) -> Result<()> {
    println!("{}", "═".repeat(60).bright_black());
    println!("{} Build and Run", "▶".cyan().bold());
    println!("{}", "═".repeat(60).bright_black());
    println!();

    // First, build the project
    println!("{} Building project...", "→".cyan());
    println!();
    crate::commands::build::execute(release, false)?;

    println!();
    println!("{}", "─".repeat(60).bright_black());
    println!();

    // Load project
    let project = Project::find_and_load()?;

    // Detect SDK
    let sdk = Sdk::detect()?;

    // Get the PDX path
    let pdx_path = project.pdx_path();

    // Verify the build output exists
    if !pdx_path.exists() {
        return Err(crate::error::CrankError::InvalidProject(format!(
            "Build output not found: {}",
            pdx_path.display()
        )));
    }

    println!("{} Launching Playdate Simulator...", "→".cyan());
    println!();
    println!("  {} {}", "Game:".dimmed(), project.config.package.name);
    println!("  {} {}", "Bundle:".dimmed(), pdx_path.display());

    // Get simulator path for display
    let sim_path = sdk.simulator_path()?;
    println!("  {} {}", "Simulator:".dimmed(), sim_path.display());
    println!();

    // Launch simulator
    sdk.launch_simulator(&pdx_path)?;

    println!("{} Simulator launched successfully!", "✓".green().bold());
    println!();
    println!(
        "{}",
        "The game is now running in the Playdate Simulator.".dimmed()
    );
    println!(
        "{}",
        "Press Ctrl+C to stop watching (simulator will keep running).".dimmed()
    );

    Ok(())
}
