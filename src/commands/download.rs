use anyhow::{Context, Error};

mod database;
mod requests;

#[derive(clap::Parser, Debug)]
pub struct Command {
    /// Set the output file
    #[arg(default_value = "squig.sqlite3", long, short)]
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
            database::sites::insert(&transaction, &site.name, &site.username)?;
            let site_id = database::sites::select(&transaction, &site.name, &site.username)?;
            for db in site.dbs {
                database::dbs::insert(&transaction, &db.folder, site_id, &db.type_)?;
            }
        }
        transaction.commit()?;

        let transaction = connection.transaction()?;
        let squigs = database::squig::select(&transaction)?;
        transaction.commit()?;

        let multi_progress = indicatif::MultiProgress::new();
        let max_title_len = squigs
            .iter()
            .map(|squig| {
                format!(
                    "{}.squig.link{}",
                    squig.username,
                    &squig.folder.strip_suffix("/").unwrap_or(&squig.folder)
                )
                .len()
            })
            .max()
            .unwrap_or(5);
        let squigs_progress_bar =
            new_progress_bar(squigs.len(), max_title_len, &multi_progress, "Total")?;

        for squig in squigs {
            let transaction = connection.transaction()?;
            let brands = requests::brands::call(&squig.username, &squig.folder)?;
            let progress_bar = new_progress_bar(
                brands.iter().map(|brand| brand.phones.len()).sum(),
                max_title_len,
                &multi_progress,
                &format!(
                    "{}.squig.link{}",
                    squig.username,
                    &squig.folder.strip_suffix("/").unwrap_or(&squig.folder)
                ),
            )?;
            for brand in brands {
                database::brands::insert(&transaction, &brand.name, squig.site_id)?;
                let brand_id = database::brands::select(&transaction, &brand.name, squig.site_id)?;
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
                    database::phones::insert(
                        &transaction,
                        phone.amazon.as_deref(),
                        brand_id,
                        &name,
                        phone.preferred_shop.as_deref(),
                        price.as_deref(),
                        phone.review_link.as_deref(),
                        review_score.as_deref(),
                        phone.shop_link.as_deref(),
                    )?;
                    let phone_id = database::phones::select(&transaction, brand_id, &name)?;
                    let texts = match phone.file {
                        requests::brands::StringOrVec::String(string) => vec![string],
                        requests::brands::StringOrVec::Vec(vec) => vec,
                    };
                    for text in texts {
                        database::files::insert(&transaction, phone_id, &text)?;
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
                            Some("L"),
                            file_id,
                            Some("Left"),
                        )?;
                        request_and_insert_other_channels(
                            &transaction,
                            &squig.username,
                            &squig.folder,
                            &text,
                            "L",
                            file_id,
                            "Left",
                        )?;
                        request_and_insert_zero_channel(
                            &transaction,
                            &squig.username,
                            &squig.folder,
                            &text,
                            Some("R"),
                            file_id,
                            Some("Right"),
                        )?;
                        request_and_insert_other_channels(
                            &transaction,
                            &squig.username,
                            &squig.folder,
                            &text,
                            "R",
                            file_id,
                            "Right",
                        )?;
                    }
                    if let Some(suffixes) = phone.suffix.map(|value| match value {
                        requests::brands::StringOrVec::String(string) => vec![string],
                        requests::brands::StringOrVec::Vec(vec) => vec,
                    }) {
                        for suffix in suffixes {
                            database::suffixes::insert(&transaction, phone_id, &suffix)?;
                        }
                    }
                    progress_bar.inc(1);
                }
            }
            squigs_progress_bar.inc(1);
            transaction.commit()?;
        }

        Ok(())
    }
}

fn new_progress_bar(
    len: usize,
    max_title_len: usize,
    multi_progress: &indicatif::MultiProgress,
    title: &str,
) -> Result<indicatif::ProgressBar, Error> {
    let progress_bar = multi_progress
        .insert_from_back(1, indicatif::ProgressBar::new(len.try_into()?))
        .with_finish(indicatif::ProgressFinish::Abandon);
    progress_bar.enable_steady_tick(std::time::Duration::from_secs_f64(0.1));
    progress_bar.set_style(
        indicatif::ProgressStyle::with_template(&format!(
            "({{pos:>4}}/{{len:>4}}) ({{percent:>3}}%) {{msg:{}}} {{elapsed_precise}} [{{wide_bar}}]",
            max_title_len
        ))?
        .progress_chars("#-"),
    );
    progress_bar.set_message(title.to_string());
    Ok(progress_bar)
}

fn request_and_insert_zero_channel(
    transaction: &rusqlite::Transaction,
    username: &str,
    folder: &str,
    text: &str,
    request_channel: Option<&str>,
    file_id: i64,
    database_channel: Option<&str>,
) -> Result<(), Error> {
    if database::channels::select(transaction, file_id, 0, database_channel).is_ok() {
        return Ok(());
    }
    if let Ok(text) = requests::channels::call(
        username,
        folder,
        &match request_channel {
            Some(value) => format!("{} {}.txt", text, value),
            None => format!("{}.txt", text),
        },
    ) {
        database::channels::insert(transaction, file_id, 0, &text, database_channel)?;
    }
    Ok(())
}

fn request_and_insert_other_channels(
    transaction: &rusqlite::Transaction,
    username: &str,
    folder: &str,
    text: &str,
    request_channel: &str,
    file_id: i64,
    database_channel: &str,
) -> Result<(), Error> {
    let mut idx = 1;
    while database::channels::select(transaction, file_id, idx, Some(database_channel)).is_ok() {
        idx += 1;
    }
    while let Ok(text) = requests::channels::call(
        username,
        folder,
        &format!("{} {}{}.txt", text, request_channel, idx),
    ) {
        database::channels::insert(transaction, file_id, idx, &text, Some(database_channel))?;
        idx += 1;
    }
    Ok(())
}
