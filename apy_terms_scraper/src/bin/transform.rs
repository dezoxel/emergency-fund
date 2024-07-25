use rusqlite::{Connection, Result};
use std::error::Error;
use async_openai::Client;
use env_logger;
use log::info;

use apy_terms_scraper::config::Config;
use apy_terms_scraper::savings_account::SavingsAccountRepo;
use apy_terms_scraper::parse_html::StrategyFactory;
use apy_terms_scraper::openai::{craft_system_prompt, extract_apy_openai_call};
use apy_terms_scraper::db::write_apy_to_db;
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
        let html_content = account.read_terms_html_from_file(&config.apy_html_path)?;
        let institution_name = account.institution_name();
        let strategy = StrategyFactory.create(&institution_name)?;
        let terms_text = strategy.extract(&html_content)?;
        let system_prompt = craft_system_prompt(&terms_text);
        let apy = extract_apy_openai_call(&client, &system_prompt).await?;
        write_apy_to_db(&conn, account.id(), apy)?;
    }

    Ok(())
}
