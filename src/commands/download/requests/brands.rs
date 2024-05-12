use anyhow::Error;
use std::collections::BTreeMap;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Brand {
    pub name: String,
    pub phones: Vec<Phone>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Phone {
    pub aliexpress: Option<String>,
    pub amazon: Option<String>,
    pub file: StringOrVec,
    pub name: StringOrVec,
    #[serde(flatten)]
    pub other: Other,
    pub preferred_shop: Option<String>,
    pub review_link: Option<String>,
    pub shop_link: Option<String>,
    pub suffix: Option<StringOrVec>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Other {
    #[serde(flatten)]
    pub price: Option<BTreeMap<String, String>>,
    #[serde(flatten)]
    pub review_score: Option<BTreeMap<String, I8OrString>>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum StringOrVec {
    String(String),
    Vec(Vec<String>),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum I8OrString {
    I8(i8),
    String(String),
}

pub fn call(username: &str, folder: &str) -> Result<Vec<Brand>, Error> {
    let url = format!(
        "https://{}.squig.link{}data/phone_book.json",
        username, folder
    );
    let response = match ureq::get(&url).call() {
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
    Ok(response.into_json::<Vec<Brand>>()?)
}
