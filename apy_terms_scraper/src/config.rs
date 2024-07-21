use serde::Deserialize;
use dotenv::dotenv;
use envy::Error;

#[derive(Deserialize)]
pub struct Config {
    pub db_path: String,
    pub apy_html_path: String,
    pub savings_account_ids_to_scrape: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        dotenv().ok();
        envy::from_env::<Self>()
    }
}