use anyhow::Error;
use indoc::indoc;

pub fn insert_or_ignore_into(
    transaction: &rusqlite::Transaction,
    file_id: i64,
    sample_index: i64,
    text: &str,
    type_: &Option<String>,
) -> Result<usize, Error> {
    let query = indoc!(
        "
        INSERT OR IGNORE INTO channels (file_id, sample_index, text, type)
        VALUES (?, ?, ?, ?)
        "
    )
    .trim_end();
    log::info!("{}\n{:?}", query, (file_id, sample_index, "...", type_));
    let result = transaction.execute(query, (file_id, sample_index, text, type_))?;
    Ok(result)
}
