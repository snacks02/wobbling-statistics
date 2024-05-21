use anyhow::Error;
use indoc::indoc;

pub fn create(transaction: &rusqlite::Transaction) -> Result<(), Error> {
    let query = indoc!(
        "
        CREATE TABLE IF NOT EXISTS brands (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            site_id INTEGER REFERENCES sites(id),
            UNIQUE(name, site_id)
        );
        CREATE INDEX IF NOT EXISTS brands_name_idx
        ON brands(name);
        CREATE INDEX IF NOT EXISTS brands_site_id_idx
        ON brands(site_id);
        "
    )
    .trim_end();
    log::info!("{}", query);
    transaction.execute_batch(query)?;
    Ok(())
}

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
