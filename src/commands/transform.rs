use anyhow::Error;
use clap::Parser;

#[derive(Debug, Parser)]
pub(crate) struct Command {}

impl Command {
    pub fn execute(&self) -> Result<(), Error> {
        Ok(())
    }
}
