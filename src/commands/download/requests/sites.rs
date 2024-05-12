use anyhow::Error;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Site {
    pub dbs: Vec<Db>,
    pub name: String,
    pub username: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Db {
    pub folder: String,
    #[serde(rename = "type")]
    pub type_: String,
}

pub fn call() -> Result<Vec<Site>, Error> {
    let url = "https://squig.link/squigsites.json";
    let response = match ureq::get(url).call() {
        Ok(response) => {
            log::info!("{} - {}", url, response.status());
            response
        }
        Err(ureq::Error::Status(code, response)) => {
            log::warn!("{} - {}", url, response.status());
            return Err(Error::from(ureq::Error::Status(code, response)));
        }
        Err(err) => return Err(Error::from(err)),
    };
    Ok(response.into_json::<Vec<Site>>()?)
}
