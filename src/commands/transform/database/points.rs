use anyhow::Error;
use indoc::indoc;

pub fn create(transaction: &rusqlite::Transaction) -> Result<(), Error> {
    let query = indoc!(
        "
        CREATE TABLE IF NOT EXISTS points (
            id            INTEGER PRIMARY KEY,
            channel_id    INTEGER REFERENCES channels(id),
            frequency_hz  REAL NOT NULL,
            idx           INTEGER NOT NULL,
            phase_degrees REAL,
            spl_db        REAL NOT NULL,
            UNIQUE(channel_id, idx)
        );
        CREATE INDEX IF NOT EXISTS points_channel_id_idx
        ON points(channel_id);
        "
    )
    .trim_end();
    log::info!("{}", query);
    transaction.execute_batch(query)?;
    Ok(())
}

pub fn insert_or_ignore(
    transaction: &rusqlite::Transaction,
    channel_id: i32,
    frequency_hz: f64,
    idx: i32,
    phase_degrees: Option<f64>,
    spl_db: f64,
) -> Result<(), Error> {
    let query = indoc!(
        "
        INSERT OR IGNORE INTO points (channel_id, frequency_hz, idx, phase_degrees, spl_db)
        VALUES (?, ?, ?, ?, ?)
        "
    )
    .trim_end();
    let params = (channel_id, frequency_hz, idx, phase_degrees, spl_db);
    log::info!("{}\n{:?}", query, params);
    transaction.execute(query, params)?;
    Ok(())
}
