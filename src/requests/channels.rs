use anyhow::Error;

pub fn call(username: &str, folder: &str, channel: &str) -> Result<String, Error> {
    let url = format!("https://{}.squig.link{}data/{}", username, folder, channel);
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
    Ok(response.into_string()?)
}
