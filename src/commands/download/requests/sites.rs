use anyhow::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Site {
    pub dbs: Vec<Db>,
    pub name: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Db {
    pub folder: String,
    #[serde(rename = "type")]
    pub type_: String,
}

pub fn call() -> Result<Vec<Site>, Error> {
    let url = "https://squig.link/squigsites.json";
    let response = match ureq::get(url).call() {
        Ok(response) => response,
        Err(ureq::Error::Status(code, response)) => {
            return Err(Error::from(ureq::Error::Status(code, response)));
        }
        Err(err) => return Err(Error::from(err)),
    };
    Ok(response.into_json::<Vec<Site>>()?)
}
