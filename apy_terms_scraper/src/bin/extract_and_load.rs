use rusqlite::{Connection, Result};
use std::error::Error;

use apy_terms_scraper::apy_terms_html::{
    download_apy_terms_html_by_url, map_ids_to_scrape_to_int, store_apy_terms_html_to_file,
};
use apy_terms_scraper::config::Config;
use apy_terms_scraper::savings_account::SavingsAccountRepo;

// TODO: provide good error messages for the most common errors
fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_env()?;
    let account_ids = map_ids_to_scrape_to_int(config.savings_account_ids_to_scrape)?;
    let conn = Connection::open(config.db_path)?;

    let savings_account_repository = SavingsAccountRepo::new(&conn);
    let accounts = savings_account_repository.find_by_ids(account_ids)?;

    for account in accounts {
        let url = account.terms_and_conditions_source_url().ok_or("Terms URL is missing")?;
        let html_content = download_apy_terms_html_by_url(url)?;
        store_apy_terms_html_to_file(&html_content, &config.apy_html_path, &account.id())?;
    }

    Ok(())
}
