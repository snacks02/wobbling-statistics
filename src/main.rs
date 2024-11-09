#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::semicolon_if_nothing_returned)]

use anyhow::Error;

mod commands;
mod measurement_parser;

#[derive(clap::Parser, Debug)]
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

#[derive(clap::Subcommand, Debug)]
enum Command {
    /// Analyze data stored in the SQLite database
    Analyze(commands::analyze::Command),

    /// Download Squiglink data and store it in the SQLite database
    Download(commands::download::Command),

    /// Transform the SQLite database to simplify analysis from SQL
    Transform(commands::transform::Command),
}

fn main() -> Result<(), Error> {
    let arguments: CommandParser = clap::Parser::parse();
    match arguments.command {
        Command::Analyze(command) => command.execute()?,
        Command::Download(command) => command.execute()?,
        Command::Transform(command) => command.execute()?,
    }

    Ok(())
}
