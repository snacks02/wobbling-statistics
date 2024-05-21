use anyhow::Error;
use indoc::indoc;

pub mod channels;
pub mod points;

pub fn init(path: &str) -> Result<rusqlite::Connection, Error> {
    let connection = rusqlite::Connection::open(path)?;

    let query = indoc!(
        "
        CREATE TABLE IF NOT EXISTS points (
            id            INTEGER PRIMARY KEY,
            channel_id    INTEGER REFERENCES channels(id),
            frequency_hz  REAL NOT NULL,
            phase_degrees REAL,
            point_index   INTEGER NOT NULL,
            spl_db        REAL NOT NULL,
            UNIQUE(channel_id, point_index)
        );
        CREATE INDEX IF NOT EXISTS points_channel_id_idx
        ON points(channel_id);
        "
    )
    .trim_end();
    log::info!("{}", query);
    connection.execute_batch(query)?;

    Ok(connection)
}
