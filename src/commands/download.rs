use anyhow::{Context, Error};
use clap::Parser;

mod database;
mod requests;

#[derive(Debug, Parser)]
pub(crate) struct Command {
    /// Set the output file
    #[arg(default_value = "squig.db", long, short)]
    output: String,
}

impl Command {
    pub fn execute(&self) -> Result<(), Error> {
        let mut connection = rusqlite::Connection::open(&self.output)?;

        let transaction = connection.transaction()?;
        database::pragmas::enable_foreign_keys(&transaction)?;
        database::sites::create(&transaction)?;
        database::dbs::create(&transaction)?;
        database::brands::create(&transaction)?;
        database::phones::create(&transaction)?;
        database::files::create(&transaction)?;
        database::channels::create(&transaction)?;
        database::suffixes::create(&transaction)?;
        transaction.commit()?;

        let transaction = connection.transaction()?;
        for site in requests::sites::call()? {
            database::sites::insert_or_ignore(&transaction, &site.name, &site.username)?;
            let site_id = database::sites::select(&transaction, &site.name, &site.username)?;
            for db in site.dbs {
                database::dbs::insert_or_ignore(&transaction, &db.folder, site_id, &db.type_)?;
            }
        }
        transaction.commit()?;

        let transaction = connection.transaction()?;
        let squigs = database::squig::select(&transaction)?;
        transaction.commit()?;
        for squig in squigs {
            let transaction = connection.transaction()?;
            for brand in requests::brands::call(&squig.username, &squig.folder)? {
                database::brands::insert_or_ignore(&transaction, &brand.name, squig.site_id)?;
                let brand_id: i64 =
                    database::brands::select(&transaction, &brand.name, squig.site_id)?;
                for phone in brand.phones {
                    let name: String = match phone.name {
                        requests::brands::StringOrVec::String(string) => string.clone(),
                        requests::brands::StringOrVec::Vec(vec) => {
                            vec.first().context("received an empty phone name")?.clone()
                        }
                    };
                    let price: Option<String> = match phone.other.price {
                        Some(value) => value.get("price").map(|value| value.to_string()),
                        None => None,
                    };
                    let review_score: Option<String> = match phone.other.review_score {
                        Some(value) => match value.get("reviewScore") {
                            Some(value) => match value {
                                requests::brands::I8OrString::I8(i8_) => Some(i8_.to_string()),
                                requests::brands::I8OrString::String(string) => {
                                    Some(string.to_string())
                                }
                            },
                            None => None,
                        },
                        None => None,
                    };
                    database::phones::insert_or_ignore(
                        &transaction,
                        &phone.amazon,
                        brand_id,
                        &name,
                        &phone.preferred_shop,
                        &price,
                        &phone.review_link,
                        &review_score,
                        &phone.shop_link,
                    )?;
                    let phone_id = database::phones::select(&transaction, brand_id, &name)?;
                    let texts = match phone.file {
                        requests::brands::StringOrVec::String(string) => vec![string],
                        requests::brands::StringOrVec::Vec(vec) => vec,
                    };
                    for text in texts {
                        database::files::insert_or_ignore(&transaction, phone_id, &text)?;
                        let file_id = database::files::select(&transaction, phone_id, &text)?;
                        request_and_insert_zero_channel(
                            &transaction,
                            &squig.username,
                            &squig.folder,
                            &text,
                            None,
                            file_id,
                            None,
                        )?;
                        request_and_insert_zero_channel(
                            &transaction,
                            &squig.username,
                            &squig.folder,
                            &text,
                            Some("L".to_string()),
                            file_id,
                            Some("Left".to_string()),
                        )?;
                        request_and_insert_other_channels(
                            &transaction,
                            &squig.username,
                            &squig.folder,
                            &text,
                            "L".to_string(),
                            file_id,
                            "Left".to_string(),
                        )?;
                        request_and_insert_zero_channel(
                            &transaction,
                            &squig.username,
                            &squig.folder,
                            &text,
                            Some("R".to_string()),
                            file_id,
                            Some("Right".to_string()),
                        )?;
                        request_and_insert_other_channels(
                            &transaction,
                            &squig.username,
                            &squig.folder,
                            &text,
                            "R".to_string(),
                            file_id,
                            "Right".to_string(),
                        )?;
                    }
                    let suffixes = match phone.suffix {
                        Some(value) => match value {
                            requests::brands::StringOrVec::String(string) => vec![string],
                            requests::brands::StringOrVec::Vec(vec) => vec,
                        },
                        None => break,
                    };
                    for suffix in suffixes {
                        database::suffixes::insert_or_ignore(&transaction, phone_id, &suffix)?;
                    }
                }
            }
            transaction.commit()?;
        }

        Ok(())
    }
}

fn request_and_insert_zero_channel(
    transaction: &rusqlite::Transaction,
    username: &str,
    folder: &str,
    text: &str,
    request_channel: Option<String>,
    file_id: i64,
    database_channel: Option<String>,
) -> Result<(), Error> {
    if let Ok(text) = requests::channels::call(
        username,
        folder,
        &match request_channel {
            Some(value) => format!("{} {}.txt", text, value),
            None => format!("{}.txt", text),
        },
    ) {
        database::channels::insert_or_ignore(transaction, file_id, 0, &text, &database_channel)?;
    }
    Ok(())
}

fn request_and_insert_other_channels(
    transaction: &rusqlite::Transaction,
    username: &str,
    folder: &str,
    text: &str,
    request_channel: String,
    file_id: i64,
    database_channel: String,
) -> Result<(), Error> {
    let mut idx = 1;
    while let Ok(text) = requests::channels::call(
        username,
        folder,
        &format!("{} {}{}.txt", text, request_channel, idx),
    ) {
        database::channels::insert_or_ignore(
            transaction,
            file_id,
            idx,
            &text,
            &Some(database_channel.clone()),
        )?;
        idx += 1;
    }
    Ok(())
}
