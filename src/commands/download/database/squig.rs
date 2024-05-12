use anyhow::Error;
use indoc::indoc;

#[derive(Debug)]
pub struct Squig {
    pub folder: String,
    pub site_id: i64,
    pub username: String,
}

pub fn select(transaction: &rusqlite::Transaction) -> Result<Vec<Squig>, Error> {
    let query = indoc!(
        "
        SELECT dbs.folder, dbs.site_id, sites.username
        FROM sites
        JOIN dbs ON dbs.site_id = sites.id
        "
    )
    .trim_end();
    let mut statement = transaction.prepare(query)?;
    log::info!("{}", query);
    let squigs = statement
        .query_map([], |row| {
            Ok(Squig {
                folder: row.get(0)?,
                site_id: row.get(1)?,
                username: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<Squig>, rusqlite::Error>>()?;
    Ok(squigs)
}
