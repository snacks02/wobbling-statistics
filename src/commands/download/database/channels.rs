use anyhow::Error;
use indoc::indoc;

pub fn create(transaction: &rusqlite::Transaction) -> Result<(), Error> {
    let query = indoc!(
        "
        CREATE TABLE IF NOT EXISTS channels (
            id           INTEGER PRIMARY KEY,
            file_id      INTEGER REFERENCES files(id),
            sample_index INTEGER NOT NULL,
            text         TEXT NOT NULL,
            type         TEXT,
            UNIQUE(file_id, text, type)
        );
        CREATE INDEX IF NOT EXISTS channels_file_id_idx
        ON channels(file_id);
        "
    )
    .trim_end();
    log::info!("{}", query);
    transaction.execute_batch(query)?;
    Ok(())
}

pub fn insert_or_ignore(
    transaction: &rusqlite::Transaction,
    file_id: i64,
    sample_index: i64,
    text: &str,
    type_: &Option<String>,
) -> Result<(), Error> {
    let query = indoc!(
        "
        INSERT OR IGNORE INTO channels (file_id, sample_index, text, type)
        VALUES (?, ?, ?, ?)
        "
    )
    .trim_end();
    log::info!("{}\n{:?}", query, (file_id, sample_index, "...", type_));
    transaction.execute(query, (file_id, sample_index, text, type_))?;
    Ok(())
}
