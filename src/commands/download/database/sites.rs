use anyhow::Error;
use indoc::indoc;

pub fn insert_or_ignore_into(
    transaction: &rusqlite::Transaction,
    name: &str,
    username: &str,
) -> Result<usize, Error> {
    let query = indoc!(
        "
        INSERT OR IGNORE INTO sites (name, username)
        VALUES (?, ?)
        "
    )
    .trim_end();
    let params = (name, username);
    log::info!("{}\n{:?}", query, params);
    let result = transaction.execute(query, params)?;
    Ok(result)
}

pub fn select_id_from(
    transaction: &rusqlite::Transaction,
    name: &str,
    username: &str,
) -> Result<i64, Error> {
    let query = "SELECT id FROM sites WHERE name = ? AND username = ?";
    let params = (name, username);
    log::info!("{}\n{:?}", query, params);
    let result = transaction.query_row(query, params, |row| row.get(0))?;
    Ok(result)
}
