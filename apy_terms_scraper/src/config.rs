use serde::Deserialize;
use dotenv::dotenv;
use std::error::Error;

pub struct Config {
    pub db_path: String,
    pub apy_html_path: String,
    pub savings_account_ids_to_scrape: Vec<i32>,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();
        let raw_config: RawConfig = envy::from_env()?;
        let parsed_ids = Config::map_ids_to_scrape_to_int(&raw_config.savings_account_ids_to_scrape)?;

        Ok(Config {
            db_path: raw_config.db_path,
            apy_html_path: raw_config.apy_html_path,
            savings_account_ids_to_scrape: parsed_ids,
        })
    }

    fn map_ids_to_scrape_to_int(ids_to_scrape_raw: &str) -> Result<Vec<i32>, Box<dyn Error>> {
        let ids_to_scrape: Vec<i32> = ids_to_scrape_raw
            .split(',')
            .map(|id| id.trim().parse().unwrap())
            .collect();
        Ok(ids_to_scrape)
    }
}

#[derive(Deserialize)]
struct RawConfig {
    pub db_path: String,
    pub apy_html_path: String,
    pub savings_account_ids_to_scrape: String,
}
