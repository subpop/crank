use anyhow::Result;
use clap::Parser;
use crank::cli::{Cli, Commands};
use crank::commands;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            name,
            path,
            template,
        } => {
            commands::new::execute(&name, path.as_deref(), template.as_deref())?;
        }
        Commands::Build { release, verbose } => {
            commands::build::execute(release, verbose)?;
        }
        Commands::Run { release } => {
            commands::run::execute(release)?;
        }
        Commands::Watch { no_run } => {
            commands::watch::execute(no_run)?;
        }
        Commands::Test { filter } => {
            commands::test::execute(filter.as_deref())?;
        }
        Commands::Clean => {
            commands::clean::execute()?;
        }
    }

    Ok(())
}
