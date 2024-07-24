use serde::Deserialize;
use dotenv::dotenv;
use std::error::Error;

use crate::vec_comma_str::comma_str_to_vec;

pub struct Config {
    pub db_path: String,
    pub apy_html_path: String,
    pub savings_account_ids_to_scrape: Vec<i32>,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();
        let raw_config: RawConfig = envy::from_env()?;
        let parsed_ids = comma_str_to_vec::<i32>(&raw_config.savings_account_ids_to_scrape)?;

        Ok(Config {
            db_path: raw_config.db_path,
            apy_html_path: raw_config.apy_html_path,
            savings_account_ids_to_scrape: parsed_ids,
        })
    }
}

#[derive(Deserialize)]
struct RawConfig {
    pub db_path: String,
    pub apy_html_path: String,
    pub savings_account_ids_to_scrape: String,
}
