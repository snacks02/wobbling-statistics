use anyhow::Error;
use indoc::indoc;

pub fn insert_or_ignore(
    transaction: &rusqlite::Transaction,
    name: &str,
    site_id: i64,
) -> Result<(), Error> {
    let query = indoc!(
        "
        INSERT OR IGNORE INTO brands (name, site_id)
        VALUES (?, ?)
        "
    )
    .trim_end();
    let params = (name, site_id);
    log::info!("{}\n{:?}", query, params);
    transaction.execute(query, params)?;
    Ok(())
}

pub fn select(transaction: &rusqlite::Transaction, name: &str, site_id: i64) -> Result<i64, Error> {
    let query = "SELECT id FROM brands WHERE name = ? AND site_id = ?";
    let params = (name, site_id);
    log::info!("{}\n{:?}", query, params);
    let result = transaction.query_row(query, params, |row| row.get(0))?;
    Ok(result)
}
