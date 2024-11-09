use anyhow::Error;
use indoc::indoc;

pub fn create(transaction: &rusqlite::Transaction) -> Result<(), Error> {
    let query = indoc!(
        "
        CREATE TABLE IF NOT EXISTS dbs (
            id      INTEGER PRIMARY KEY,
            folder  TEXT NOT NULL,
            site_id INTEGER REFERENCES sites(id),
            type    TEXT NOT NULL,
            UNIQUE(folder, site_id, type)
        );
        CREATE INDEX IF NOT EXISTS dbs_folder_idx
        ON dbs(folder);
        CREATE INDEX IF NOT EXISTS dbs_site_id_idx
        ON dbs(site_id);
        CREATE INDEX IF NOT EXISTS dbs_type_idx
        ON dbs(type);
        "
    )
    .trim_end();
    transaction.execute_batch(query)?;
    Ok(())
}

pub fn insert(
    transaction: &rusqlite::Transaction,
    folder: &str,
    site_id: i64,
    type_: &str,
) -> Result<(), Error> {
    let query = indoc!(
        "
        INSERT OR IGNORE INTO dbs (folder, site_id, type)
        VALUES (?, ?, ?)
        "
    )
    .trim_end();
    let params = (folder, site_id, type_);
    transaction.execute(query, params)?;
    Ok(())
}
