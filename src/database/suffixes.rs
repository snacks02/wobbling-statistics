use anyhow::Error;
use indoc::indoc;

pub fn insert_or_ignore_into(
    transaction: &rusqlite::Transaction,
    phone_id: i64,
    text: &str,
) -> Result<usize, Error> {
    let query = indoc!(
        "
        INSERT OR IGNORE INTO suffixes (phone_id, text)
        VALUES (?, ?)
        "
    )
    .trim_end();
    let params = (phone_id, text);
    log::info!("{}\n{:?}", query, params);
    let result = transaction.execute(query, params)?;
    Ok(result)
}
