use anyhow::Error;
use indoc::indoc;

pub fn insert_or_ignore(
    transaction: &rusqlite::Transaction,
    channel_id: i32,
    frequency_hz: f64,
    phase_degrees: Option<f64>,
    point_index: i32,
    spl_db: f64,
) -> Result<(), Error> {
    let query = indoc!(
        "
        INSERT OR IGNORE INTO points (channel_id, frequency_hz, phase_degrees, point_index, spl_db)
        VALUES (?, ?, ?, ?, ?)
        "
    )
    .trim_end();
    let params = (channel_id, frequency_hz, phase_degrees, point_index, spl_db);
    log::info!("{}\n{:?}", query, params);
    transaction.execute(query, params)?;
    Ok(())
}
