use anyhow::Error;
use clap::Parser;
use std::collections;
use std::fs;

use crate::measurement_parser;

mod database;

#[derive(Debug, Parser)]
pub struct Command {
    /// Set the input file
    #[arg(default_value = "squig.sqlite3", long, short)]
    input: String,

    /// Set the output directory
    #[arg(default_value = "out", long, short)]
    output: String,
}

struct Bar {
    brand_name: String,
    average_channel_imbalance: f64,
}

impl Command {
    pub fn execute(&self) -> Result<(), Error> {
        let mut connection = rusqlite::Connection::open(&self.input)?;
        fs::create_dir_all(&self.output)?;

        let transaction = connection.transaction()?;
        let mut brand_differences = collections::HashMap::new();
        for output in database::channel_imbalance::select(&transaction)? {
            let values = brand_differences
                .entry(output.brand_name)
                .or_insert(Vec::<f64>::new());
            let left_channel_points = measurement_parser::parse(&output.left_channel_text)?;
            let right_channel_points = measurement_parser::parse(&output.right_channel_text)?;
            if left_channel_points
                .iter()
                .zip(&right_channel_points)
                .any(|(l, r)| l.frequency_hz != r.frequency_hz)
            {
                continue;
            }
            let left_channel_average = left_channel_points
                .iter()
                .map(|point| point.spl_db)
                .sum::<f64>()
                / left_channel_points.len() as f64;
            let right_channel_average = right_channel_points
                .iter()
                .map(|point| point.spl_db)
                .sum::<f64>()
                / right_channel_points.len() as f64;
            values.push((left_channel_average - right_channel_average).abs());
        }
        let mut brand_differences: Vec<Bar> = brand_differences
            .iter()
            .map(|(name, points)| {
                let average_channel_imbalance = points.iter().sum::<f64>() / points.len() as f64;
                Bar {
                    brand_name: format!(
                        "{} ({}) (~{:.2} dB)",
                        name,
                        points.len(),
                        average_channel_imbalance
                    ),
                    average_channel_imbalance,
                }
            })
            .collect();
        brand_differences.sort_by(|a, b| {
            a.average_channel_imbalance
                .total_cmp(&b.average_channel_imbalance)
        });
        let keys: Vec<String> = brand_differences
            .iter()
            .map(|brand_difference| brand_difference.brand_name.clone())
            .collect();
        let values: Vec<f64> = brand_differences
            .iter()
            .map(|brand_difference| brand_difference.average_channel_imbalance)
            .collect();
        let mut plot = plotly::Plot::new();
        plot.add_trace(
            plotly::Bar::new(values.clone(), keys.clone())
                .marker(
                    plotly::common::Marker::new()
                        .color(plotly::common::color::Rgb::new(58, 124, 191)),
                )
                .orientation(plotly::common::Orientation::Horizontal),
        );
        plot.set_layout(
            plotly::Layout::new()
                .bar_gap(0.1)
                .height(keys.len() * 30)
                .margin(
                    plotly::layout::Margin::new()
                        .bottom(14)
                        .left(200)
                        .right(0)
                        .top(0),
                ),
        );
        plot.write_image(
            format!("{}/channel_imbalance.svg", &self.output),
            plotly::ImageFormat::SVG,
            1080,
            keys.len() * 30,
            1.0,
        );
        transaction.commit()?;

        Ok(())
    }
}
