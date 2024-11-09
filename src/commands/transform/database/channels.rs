use anyhow::Error;
use indoc::indoc;

#[derive(Debug)]
pub struct IdAndText {
    pub id: i32,
    pub text: String,
}

pub fn drop_column_text(transaction: &rusqlite::Transaction) -> Result<(), Error> {
    let query = indoc!(
        "
        ALTER TABLE channels
        RENAME TO old_channels;

        CREATE TABLE IF NOT EXISTS channels (
            id      INTEGER PRIMARY KEY,
            file_id INTEGER REFERENCES files(id),
            idx     INTEGER NOT NULL,
            type    TEXT
        );

        CREATE INDEX IF NOT EXISTS channels_file_id_idx
        ON channels(file_id);

        INSERT INTO channels
        SELECT id, file_id, idx, type
        FROM old_channels;

        DROP TABLE old_channels;
        "
    )
    .trim_end();
    transaction.execute_batch(query)?;
    Ok(())
}

pub fn select(transaction: &rusqlite::Transaction) -> Result<Vec<IdAndText>, Error> {
    let query = "SELECT id, text FROM channels";
    let mut statement = transaction.prepare(query)?;
    let channel_iter = statement.query_map([], |row| {
        Ok(IdAndText {
            id: row.get(0)?,
            text: row.get(1)?,
        })
    })?;
    let id_and_texts = channel_iter.collect::<Result<Vec<IdAndText>, rusqlite::Error>>()?;
    Ok(id_and_texts)
}
