use anyhow::Error;
use indoc::indoc;

pub fn insert_or_ignore(
    transaction: &rusqlite::Transaction,
    phone_id: i64,
    text: &str,
) -> Result<(), Error> {
    let query = indoc!(
        "
        INSERT OR IGNORE INTO suffixes (phone_id, text)
        VALUES (?, ?)
        "
    )
    .trim_end();
    let params = (phone_id, text);
    log::info!("{}\n{:?}", query, params);
    transaction.execute(query, params)?;
    Ok(())
}
