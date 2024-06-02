use anyhow::Error;
use clap::{Parser, Subcommand};

mod commands;
mod logger;

#[derive(Debug, Parser)]
#[command(
    disable_help_subcommand = true,
    disable_version_flag = true,
    version = env!("CARGO_PKG_VERSION")
)]
struct CommandParser {
    /// Show version
    #[arg(action = clap::builder::ArgAction::Version, short, long)]
    version: (),

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Analyze data stored in the SQLite database
    Analyze(commands::analyze::Command),

    /// Download Squiglink data and store it in the SQLite database
    Download(commands::download::Command),

    /// Transform the SQLite database to simplify analysis from SQL
    Transform(commands::transform::Command),
}

fn main() -> Result<(), Error> {
    logger::init()?;

    let arguments = CommandParser::parse();
    match arguments.command {
        Command::Analyze(command) => command.execute()?,
        Command::Download(command) => command.execute()?,
        Command::Transform(command) => command.execute()?,
    }

    Ok(())
}
