use anyhow::Error;
use indoc::indoc;

#[derive(Debug)]
pub struct Output {
    pub brand_name: String,
    pub left_channel_text: String,
    pub right_channel_text: String,
}

pub fn select(transaction: &rusqlite::Transaction) -> Result<Vec<Output>, Error> {
    let query = indoc!(
        "
        SELECT brands.name, left_channels.text AS left_channel_text, right_channels.text AS right_channel_text
        FROM brands
        JOIN phones ON phones.brand_id = brands.id
        JOIN files ON files.phone_id = phones.id
        JOIN channels left_channels ON left_channels.file_id = files.id
        JOIN channels right_channels ON right_channels.file_id = left_channels.file_id
        AND left_channels.id < right_channels.id
        AND left_channels.idx = right_channels.idx
        AND left_channels.text != right_channels.text
        AND left_channels.type != right_channels.type
        "
    )
    .trim_end();
    let mut statement = transaction.prepare(query)?;
    let outputs = statement
        .query_map([], |row| {
            Ok(Output {
                brand_name: row.get(0)?,
                left_channel_text: row.get(1)?,
                right_channel_text: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<Output>, rusqlite::Error>>()?;
    Ok(outputs)
}
