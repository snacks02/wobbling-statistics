use anyhow::Error;

pub fn enable_foreign_keys(transaction: &rusqlite::Transaction) -> Result<(), Error> {
    log::info!("PRAGMA foreign_keys = ON;");
    transaction.pragma_update(None, "foreign_keys", "ON")?;
    Ok(())
}
