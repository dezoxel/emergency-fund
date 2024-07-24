use rusqlite::{Connection, Result};
use std::error::Error;

use apy_terms_scraper::config::Config;
use apy_terms_scraper::savings_account::SavingsAccountRepo;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_env()?;
    let conn = Connection::open(config.db_path)?;

    let savings_account_repo = SavingsAccountRepo::new(&conn);
    let accounts = savings_account_repo.find_by_ids(config.savings_account_ids_to_scrape)?;

    for account in accounts {
        let html = account.download_terms_html()?;
        account.write_terms_html_to_file(&config.apy_html_path, &html)?;
    }

    Ok(())
}
