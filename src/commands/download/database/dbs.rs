use anyhow::Error;
use indoc::indoc;

pub fn insert_or_ignore(
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
    log::info!("{}\n{:?}", query, params);
    transaction.execute(query, params)?;
    Ok(())
}
