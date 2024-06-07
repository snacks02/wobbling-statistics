use anyhow::Error;
use clap::Parser;
use std::fs;

use crate::measurement_parser;

mod database;

#[derive(Debug, Parser)]
pub struct Command {
    /// Set the input file
    #[arg(default_value = "squig.sqlite3", long, short)]
    input: String,

    /// Set the output file
    #[arg(default_value = "squig_transformed.sqlite3", long, short)]
    output: String,
}

impl Command {
    pub fn execute(&self) -> Result<(), Error> {
        fs::copy(&self.input, &self.output)?;
        let mut connection = rusqlite::Connection::open(&self.output)?;

        let transaction = connection.transaction()?;
        let channels = database::channels::select(&transaction)?;
        database::channels::drop_column_text(&transaction)?;
        database::points::create(&transaction)?;
        for channel in channels {
            let points = measurement_parser::parse(&channel.text)?;
            for (index, point) in points.iter().enumerate() {
                database::points::insert(
                    &transaction,
                    channel.id,
                    point.frequency_hz,
                    i32::try_from(index)?,
                    point.phase_degrees,
                    point.spl_db,
                )?;
            }
        }
        transaction.commit()?;

        Ok(())
    }
}
