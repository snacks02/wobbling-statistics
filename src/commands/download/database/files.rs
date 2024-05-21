use anyhow::Error;
use indoc::indoc;

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
