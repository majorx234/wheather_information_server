use crate::{config::Config, error::Error};
use reqwest;
use reqwest::blocking::Client;
use reqwest::Url;

struct PageScraper {
    client: Client,
}

impl PageScraper {
    pub fn new() -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/geo+json".parse().unwrap());
        let client = reqwest::blocking::Client::builder()
            .user_agent(Config::new().get_app_user_agent())
            .default_headers(headers)
            .build()
            .expect("couldn't create requests client");
        PageScraper { client }
    }

    pub fn extract_temperature_from(url: &String) -> Result<i32, Error> {
        let url = Url::parse(url)?;
        Ok(12)
    }
}