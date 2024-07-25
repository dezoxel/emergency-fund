use log::{info, error};
use env_logger;
use rusqlite::{Connection, Result};
use std::error::Error;

use apy_terms_scraper::config::Config;
use apy_terms_scraper::savings_account::SavingsAccountRepo;
use apy_terms_scraper::vec_comma_str::vec_to_comma_str;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_env()?;
    env_logger::init();
    let conn = Connection::open(config.db_path)?;

    let savings_account_repo = SavingsAccountRepo::new(&conn);
    let accounts = savings_account_repo.find_by_ids(&config.savings_account_ids_to_scrape)?;
    info!(
        "Found {} accounts to scrape: {}.",
        accounts.len(),
        vec_to_comma_str(&config.savings_account_ids_to_scrape)
    );

    for account in accounts {
        let html = account.download_terms_html().map_err(|e| {
            error!("Failed to download terms HTML for account {}: {}", account.id(), e);
            e
        })?;

        account
            .write_terms_html_to_file(&config.apy_html_path, &html)
            .map_err(|e| {
                error!("Failed to write terms HTML for account {}: {}", account.id(), e);
                e
            })?;
    }

    Ok(())
}
