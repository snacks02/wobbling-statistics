use anyhow::Error;
use indoc::indoc;

pub fn create(transaction: &rusqlite::Transaction) -> Result<(), Error> {
    let query = indoc!(
        "
        CREATE TABLE IF NOT EXISTS files (
            id       INTEGER PRIMARY KEY,
            phone_id INTEGER REFERENCES phones(id),
            text     TEXT NOT NULL,
            UNIQUE(phone_id, text)
        );
        CREATE INDEX IF NOT EXISTS files_phone_id_idx
        ON files(phone_id);
        CREATE INDEX IF NOT EXISTS files_text_idx
        ON files(text);
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
        INSERT OR IGNORE INTO files (phone_id, text)
        VALUES (?, ?)
        "
    )
    .trim_end();
    let params = (phone_id, text);
    log::info!("{}\n{:?}", query, params);
    transaction.execute(query, params)?;
    Ok(())
}

pub fn select(
    transaction: &rusqlite::Transaction,
    phone_id: i64,
    text: &str,
) -> Result<i64, Error> {
    let query = "SELECT id FROM files WHERE phone_id = ? AND text = ?";
    let params = (phone_id, text);
    log::info!("{}\n{:?}", query, params);
    let result = transaction.query_row(query, params, |row| row.get(0))?;
    Ok(result)
}
