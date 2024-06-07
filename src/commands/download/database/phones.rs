use anyhow::Error;
use indoc::indoc;

pub fn create(transaction: &rusqlite::Transaction) -> Result<(), Error> {
    let query = indoc!(
        "
        CREATE TABLE IF NOT EXISTS phones (
            id             INTEGER PRIMARY KEY,
            amazon         TEXT,
            brand_id       INTEGER REFERENCES brands(id),
            name           TEXT NOT NULL,
            preferred_shop TEXT,
            price          TEXT,
            review_link    TEXT,
            review_score   TEXT,
            shop_link      TEXT,
            UNIQUE(brand_id, name)
        );
        CREATE INDEX IF NOT EXISTS phones_brand_id_idx
        ON phones(brand_id);
        CREATE INDEX IF NOT EXISTS phones_name_idx
        ON phones(name);
        "
    )
    .trim_end();
    log::info!("{}", query);
    transaction.execute_batch(query)?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn insert(
    transaction: &rusqlite::Transaction,
    amazon: Option<&str>,
    brand_id: i64,
    name: &str,
    preferred_shop: Option<&str>,
    price: Option<&str>,
    review_link: Option<&str>,
    review_score: Option<&str>,
    shop_link: Option<&str>,
) -> Result<(), Error> {
    let query = indoc!(
        "
        INSERT OR IGNORE INTO phones (
            amazon,
            brand_id,
            name,
            preferred_shop,
            price,
            review_link,
            review_score,
            shop_link
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "
    )
    .trim_end();
    let params = (
        amazon,
        brand_id,
        name,
        preferred_shop,
        price,
        review_link,
        review_score,
        shop_link,
    );
    log::info!("{}\n{:?}", query, params);
    transaction.execute(query, params)?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn select(
    transaction: &rusqlite::Transaction,
    brand_id: i64,
    name: &str,
) -> Result<i64, Error> {
    let query = "SELECT id FROM phones WHERE brand_id = ? AND name = ?";
    let params = (brand_id, name);
    log::info!("{}\n{:?}", query, params);
    let result = transaction.query_row(query, params, |row| row.get(0))?;
    Ok(result)
}
