use anyhow::Error;
use indoc::indoc;

pub fn insert_or_ignore_into(
    transaction: &rusqlite::Transaction,
    phone_id: i64,
    text: &str,
) -> Result<usize, Error> {
    let query = indoc!(
        "
        INSERT OR IGNORE INTO files (phone_id, text)
        VALUES (?, ?)
        "
    )
    .trim_end();
    let params = (phone_id, text);
    log::info!("{}\n{:?}", query, params);
    let result = transaction.execute(query, params)?;
    Ok(result)
}

pub fn select_id_from(
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
