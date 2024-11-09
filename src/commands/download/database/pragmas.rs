use anyhow::Error;

pub fn enable_foreign_keys(transaction: &rusqlite::Transaction) -> Result<(), Error> {
    transaction.pragma_update(None, "foreign_keys", "ON")?;
    Ok(())
}
