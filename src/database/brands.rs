use anyhow::Error;
use indoc::indoc;

pub fn insert_or_ignore_into(
    transaction: &rusqlite::Transaction,
    name: &str,
    site_id: i64,
) -> Result<usize, Error> {
    let query = indoc!(
        "
        INSERT OR IGNORE INTO brands (name, site_id)
        VALUES (?, ?)
        "
    )
    .trim_end();
    let params = (name, site_id);
    log::info!("{}\n{:?}", query, params);
    let result = transaction.execute(query, params)?;
    Ok(result)
}

pub fn select_id_from(
    transaction: &rusqlite::Transaction,
    name: &str,
    site_id: i64,
) -> Result<i64, Error> {
    let query = "SELECT id FROM brands WHERE name = ? AND site_id = ?";
    let params = (name, site_id);
    log::info!("{}\n{:?}", query, params);
    let result = transaction.query_row(query, params, |row| row.get(0))?;
    Ok(result)
}
