//! Watch for file changes and rebuild automatically

use crate::error::Result;
use crate::project::Project;
use colored::Colorize;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

/// Execute the 'watch' command
pub fn execute(no_run: bool) -> Result<()> {
    let project = Project::find_and_load()?;

    println!("{} for changes...", "Watching".cyan().bold());
    println!("  Source: {}", project.source_dir().display());
    println!("  Assets: {}", project.assets_dir().display());
    println!();
    println!("Press Ctrl+C to stop");
    println!();

    // Build once initially
    if let Err(e) = crate::commands::build::execute(false, false) {
        eprintln!("{} Initial build failed: {}", "✗".red().bold(), e);
    } else if !no_run {
        // Launch simulator on first build
        if let Err(e) = launch_simulator_for_project(&project) {
            eprintln!("{} Failed to launch simulator: {}", "✗".red().bold(), e);
        }
    }

    // Set up file watcher
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res: std::result::Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        Config::default().with_poll_interval(Duration::from_millis(500)),
    )?;

    // Watch source and assets directories
    watcher.watch(project.source_dir().as_ref(), RecursiveMode::Recursive)?;
    watcher.watch(project.assets_dir().as_ref(), RecursiveMode::Recursive)?;

    // Watch for changes
    let mut last_build = std::time::Instant::now();

    loop {
        match rx.recv() {
            Ok(event) => {
                // Debounce: only rebuild if 500ms has passed since last build
                if last_build.elapsed() < Duration::from_millis(500) {
                    continue;
                }

                // Filter out non-modify events
                if !is_relevant_event(&event) {
                    continue;
                }

                println!();
                println!("{} Change detected, rebuilding...", "»".cyan().bold());

                last_build = std::time::Instant::now();

                if let Err(e) = crate::commands::build::execute(false, false) {
                    eprintln!("{} Build failed: {}", "✗".red().bold(), e);
                } else {
                    println!();
                }
            }
            Err(e) => {
                eprintln!("Watch error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

fn is_relevant_event(event: &Event) -> bool {
    use notify::EventKind;

    match event.kind {
        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
            // Only watch for source files
            event.paths.iter().any(|p| {
                p.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| matches!(e, "lua" | "png" | "wav" | "fnt" | "json"))
                    .unwrap_or(false)
            })
        }
        _ => false,
    }
}

fn launch_simulator_for_project(project: &Project) -> Result<()> {
    let sdk = crate::sdk::Sdk::detect()?;
    let pdx_path = project.pdx_path();
    sdk.launch_simulator(&pdx_path)
}
