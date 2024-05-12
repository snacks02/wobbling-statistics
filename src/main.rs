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
    /// Download Squiglink data and save it into a SQLite database
    Download(commands::download::Command),
}

fn main() -> Result<(), Error> {
    logger::init()?;

    let arguments = CommandParser::parse();
    match arguments.command {
        Command::Download(command) => command.execute()?,
    }

    Ok(())
}
