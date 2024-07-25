use async_openai::Client;
use env_logger;
use log::{error, info};
use rusqlite::{Connection, Result};
use std::error::Error;

use apy_terms_scraper::config::Config;
use apy_terms_scraper::db::write_apy_to_db;
use apy_terms_scraper::savings_account::SavingsAccountRepo;
use apy_terms_scraper::vec_comma_str::vec_to_comma_str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_env()?;
    env_logger::init();
    let client = Client::new();
    let conn = Connection::open(config.db_path)?;

    let savings_account_repo = SavingsAccountRepo::new(&conn);
    let accounts = savings_account_repo.find_by_ids(&config.savings_account_ids_to_scrape)?;
    info!(
        "Found {} accounts to scrape: {}.",
        accounts.len(),
        vec_to_comma_str(&config.savings_account_ids_to_scrape)
    );

    for account in accounts {
        let html = account
            .read_terms_html_from_file(&config.apy_html_path)
            .map_err(|e| {
                error!("Failed to read terms HTML from file for account {}: {}", account.id(), e);
                e
            })?;
        let terms_text = account
            .extract_terms_text_from_html(&html)
            .map_err(|e| {
                error!("Failed to extract terms text from HTML for account {}: {}", account.id(), e);
                e
            })?;
        let apy = account
            .extract_apy_from_terms_text(&client, &terms_text)
            .await
            .map_err(|e| {
                error!("Failed to extract APY from terms text for account {}: {}", account.id(), e);
                e
            })?;
        write_apy_to_db(&conn, account.id(), apy)?;
    }

    Ok(())
}
