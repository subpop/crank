//! CLI argument definitions using clap

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "crank")]
#[command(author, version, about, long_about = None)]
#[command(
    about = "crank - A build tool for Playdate game development",
    long_about = "crank is a command-line build tool for Playdate game development, \
                  inspired by Cargo. It provides project scaffolding, build automation, \
                  and development tools to streamline your workflow."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new Playdate project
    #[command(visible_alias = "n")]
    New {
        /// Name of the project
        name: String,

        /// Path where the project will be created (defaults to current directory)
        path: Option<String>,

        /// Project template to use
        #[arg(short, long, default_value = "lua-basic")]
        template: Option<String>,
    },

    /// Build the current project
    #[command(visible_alias = "b")]
    Build {
        /// Build in release mode with optimizations
        #[arg(short, long)]
        release: bool,

        /// Show detailed build output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Build and run the project in the Playdate Simulator
    #[command(visible_alias = "r")]
    Run {
        /// Run release build
        #[arg(short, long)]
        release: bool,
    },

    /// Watch for changes and rebuild automatically
    #[command(visible_alias = "w")]
    Watch {
        /// Don't launch simulator, just rebuild on changes
        #[arg(long)]
        no_run: bool,
    },

    /// Run tests
    #[command(visible_alias = "t")]
    Test {
        /// Filter tests by pattern
        #[arg(short, long)]
        filter: Option<String>,
    },

    /// Clean build artifacts
    #[command(visible_alias = "c")]
    Clean,
}
