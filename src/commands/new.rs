//! Create new Playdate projects

use crate::error::Result;
use crate::project::create_new_project;
use colored::Colorize;
use std::env;
use std::process::Command;

/// Execute the 'new' command
pub fn execute(name: &str, path: Option<&str>, template: Option<&str>) -> Result<()> {
    let template = template.unwrap_or("lua-basic");

    // Determine the base directory where the project will be created
    let base_dir = if let Some(p) = path {
        std::path::PathBuf::from(p)
    } else {
        env::current_dir()?
    };

    println!(
        "{} {} project '{}'...",
        "Creating".green().bold(),
        template.cyan(),
        name.bold()
    );

    if path.is_some() {
        println!("  Location: {}", base_dir.display());
    }

    // Create project in specified or current directory
    let project = create_new_project(name, &base_dir, template)?;

    // Try to initialize git repository (optional, don't fail if git is not available)
    let git_init_success = init_git_repo(&project.root);

    // Check if playdate-luacats was cloned
    let luacats_exists = project.root.join("playdate-luacats").exists();

    println!("{} Created new Playdate project!", "✓".green().bold());
    println!();
    println!("Project structure:");
    println!("  {}/", name.cyan());
    println!("  ├── Playdate.toml");
    println!("  ├── README.md");
    println!("  ├── source/");
    println!("  │   ├── main.lua");
    println!("  │   ├── pdxinfo");
    println!("  │   └── .luarc.json");
    println!("  ├── assets/");
    println!("  │   ├── images/");
    println!("  │   ├── sounds/");
    println!("  │   └── fonts/");
    if luacats_exists {
        println!("  ├── playdate-luacats/  (Lua LSP type definitions)");
    }
    println!("  └── tests/");
    println!("      └── test_basic.lua");

    if git_init_success {
        println!();
        println!("{} Initialized git repository", "✓".green().bold());
    }

    if luacats_exists {
        println!(
            "{} Installed playdate-luacats for IDE support",
            "✓".green().bold()
        );
    }

    println!();
    println!("{}", "Next steps:".bold());
    println!("  {} {}", "cd".cyan(), name);
    println!("  {} {}", "crank".cyan(), "build".yellow());
    println!("  {} {}", "crank".cyan(), "run".yellow());
    println!();
    println!("{}", "Happy game development! 🎮".bright_green());

    Ok(())
}

/// Initialize a git repository in the project directory
fn init_git_repo(project_path: &std::path::Path) -> bool {
    // Check if git is available
    let git_check = Command::new("git").arg("--version").output();

    if git_check.is_err() {
        return false;
    }

    // Initialize git repo
    let git_init = Command::new("git")
        .arg("init")
        .current_dir(project_path)
        .output();

    if git_init.is_err() {
        return false;
    }

    // Create .gitignore for Playdate projects
    let gitignore_content = "# Build artifacts\nbuild/\n*.pdx\n\n# System files\n.DS_Store\nThumbs.db\n\n# Editor files\n.vscode/\n.idea/\n*.swp\n*.swo\n*~\n\n# Dependencies\nplaydate-luacats/\n";

    if std::fs::write(project_path.join(".gitignore"), gitignore_content).is_err() {
        return false;
    }

    true
}
