use anyhow::Error;
use indoc::indoc;

pub fn create(transaction: &rusqlite::Transaction) -> Result<(), Error> {
    let query = indoc!(
        "
        CREATE TABLE IF NOT EXISTS channels (
            id      INTEGER PRIMARY KEY,
            file_id INTEGER REFERENCES files(id),
            idx     INTEGER NOT NULL,
            text    TEXT NOT NULL,
            type    TEXT,
            UNIQUE(file_id, text, type)
        );
        CREATE INDEX IF NOT EXISTS channels_file_id_idx
        ON channels(file_id);
        CREATE INDEX IF NOT EXISTS channels_idx_idx
        ON channels(idx);
        CREATE INDEX IF NOT EXISTS channels_type_idx
        ON channels(type);
        "
    )
    .trim_end();
    log::info!("{}", query);
    transaction.execute_batch(query)?;
    Ok(())
}

pub fn insert(
    transaction: &rusqlite::Transaction,
    file_id: i64,
    idx: i64,
    text: &str,
    type_: Option<&str>,
) -> Result<(), Error> {
    let query = indoc!(
        "
        INSERT OR IGNORE INTO channels (file_id, idx, text, type)
        VALUES (?, ?, ?, ?)
        "
    )
    .trim_end();
    log::info!("{}\n{:?}", query, (file_id, idx, "...", type_));
    transaction.execute(query, (file_id, idx, text, type_))?;
    Ok(())
}

pub fn select(
    transaction: &rusqlite::Transaction,
    file_id: i64,
    idx: i64,
    type_: Option<&str>,
) -> Result<i64, Error> {
    let query = "SELECT id FROM channels WHERE file_id = ? AND idx = ? AND type = ?";
    let params = (file_id, idx, type_);
    log::info!("{}\n{:?}", query, params);
    let result = transaction.query_row(query, params, |row| row.get(0))?;
    Ok(result)
}
