use anyhow::Error;
use indoc::indoc;

pub fn create(transaction: &rusqlite::Transaction) -> Result<(), Error> {
    let query = indoc!(
        "
        CREATE TABLE IF NOT EXISTS suffixes (
            id       INTEGER PRIMARY KEY,
            phone_id INTEGER REFERENCES phones(id),
            text     TEXT NOT NULL,
            UNIQUE(phone_id, text)
        );
        CREATE INDEX IF NOT EXISTS suffixes_phone_id_idx
        ON suffixes(phone_id);
        "
    )
    .trim_end();
    log::info!("{}", query);
    transaction.execute_batch(query)?;
    Ok(())
}

pub fn insert_or_ignore(
    transaction: &rusqlite::Transaction,
    phone_id: i64,
    text: &str,
) -> Result<(), Error> {
    let query = indoc!(
        "
        INSERT OR IGNORE INTO suffixes (phone_id, text)
        VALUES (?, ?)
        "
    )
    .trim_end();
    let params = (phone_id, text);
    log::info!("{}\n{:?}", query, params);
    transaction.execute(query, params)?;
    Ok(())
}
