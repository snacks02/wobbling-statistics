use anyhow::Error;
use clap::Parser;
use std::fs;

mod database;
mod parser;

#[derive(Debug, Parser)]
pub(crate) struct Command {
    /// Set the input file
    #[arg(default_value = "squig.db", long, short)]
    input: String,

    /// Set the output file
    #[arg(default_value = "squig_transformed.db", long, short)]
    output: String,
}

impl Command {
    pub fn execute(&self) -> Result<(), Error> {
        fs::copy(&self.input, &self.output)?;
        let mut connection = database::init(&self.output)?;

        let transaction = connection.transaction()?;
        let channels = database::channels::select(&transaction)?;
        database::channels::drop_column_text(&transaction)?;
        for channel in channels {
            let points = parser::parse(&channel.text)?;
            for (index, point) in points.iter().enumerate() {
                database::points::insert_or_ignore_into(
                    &transaction,
                    channel.id,
                    point.frequency_hz,
                    point.phase_degrees,
                    i32::try_from(index)?,
                    point.spl_db,
                )?;
            }
        }
        transaction.commit()?;

        Ok(())
    }
}
