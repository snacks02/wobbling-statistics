use anyhow::Error;
use indoc::indoc;

pub mod brands;
pub mod channels;
pub mod dbs;
pub mod files;
pub mod phones;
pub mod sites;
pub mod squig;
pub mod suffixes;

pub fn init(path: &str) -> Result<rusqlite::Connection, Error> {
    let connection = rusqlite::Connection::open(path)?;

    log::info!("PRAGMA foreign_keys = ON;");
    connection.pragma_update(None, "foreign_keys", "ON")?;

    let query = indoc!(
        "
        CREATE TABLE IF NOT EXISTS sites (
            id       INTEGER PRIMARY KEY,
            name     TEXT NOT NULL,
            username TEXT NOT NULL,
            UNIQUE(name, username)
        );
        CREATE INDEX IF NOT EXISTS sites_name_idx
        ON sites(name);
        CREATE INDEX IF NOT EXISTS sites_username_idx
        ON sites(username);

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

        CREATE TABLE IF NOT EXISTS brands (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            site_id INTEGER REFERENCES sites(id),
            UNIQUE(name, site_id)
        );
        CREATE INDEX IF NOT EXISTS brands_name_idx
        ON brands(name);
        CREATE INDEX IF NOT EXISTS brands_site_id_idx
        ON brands(site_id);

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

        CREATE TABLE IF NOT EXISTS files (
            id       INTEGER PRIMARY KEY,
            phone_id INTEGER REFERENCES phones(id),
            text     TEXT NOT NULL,
            UNIQUE(phone_id, text)
        );
        CREATE INDEX IF NOT EXISTS files_phone_id_idx
        ON files(phone_id);
        CREATE INDEX IF NOT EXISTS files_text_idx
        ON files(text);

        CREATE TABLE IF NOT EXISTS channels (
            id           INTEGER PRIMARY KEY,
            file_id      INTEGER REFERENCES files(id),
            sample_index INTEGER NOT NULL,
            text         TEXT NOT NULL,
            type         TEXT,
            UNIQUE(file_id, text, type)
        );
        CREATE INDEX IF NOT EXISTS channels_file_id_idx
        ON channels(file_id);

        CREATE TABLE IF NOT EXISTS suffixes (
            id       INTEGER PRIMARY KEY,
            phone_id INTEGER REFERENCES phones(id),
            text     TEXT NOT NULL,
            UNIQUE(phone_id, text)
        );
        CREATE INDEX IF NOT EXISTS suffixes_phone_id_idx
        ON suffixes(phone_id);
        "
    )
    .trim_end();
    log::info!("{}", query);
    connection.execute_batch(query)?;

    Ok(connection)
}
