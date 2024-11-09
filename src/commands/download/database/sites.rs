use anyhow::Error;
use indoc::indoc;

pub fn create(transaction: &rusqlite::Transaction) -> Result<(), Error> {
    let query = indoc!(
        "
        CREATE TABLE IF NOT EXISTS sites (
            id       INTEGER PRIMARY KEY,
            name     TEXT NOT NULL,
            username TEXT NOT NULL,
            UNIQUE(name, username)
        );
        CREATE INDEX IF NOT EXISTS sites_name_idx
        ON sites(name);
        CREATE INDEX IF NOT EXISTS sites_username_idx
        ON sites(username);
        "
    )
    .trim_end();
    transaction.execute_batch(query)?;
    Ok(())
}

pub fn insert(
    transaction: &rusqlite::Transaction,
    name: &str,
    username: &str,
) -> Result<(), Error> {
    let query = indoc!(
        "
        INSERT OR IGNORE INTO sites (name, username)
        VALUES (?, ?)
        "
    )
    .trim_end();
    let params = (name, username);
    transaction.execute(query, params)?;
    Ok(())
}

pub fn select(
    transaction: &rusqlite::Transaction,
    name: &str,
    username: &str,
) -> Result<i64, Error> {
    let query = "SELECT id FROM sites WHERE name = ? AND username = ?";
    let params = (name, username);
    let result = transaction.query_row(query, params, |row| row.get(0))?;
    Ok(result)
}
