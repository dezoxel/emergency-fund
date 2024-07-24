use rusqlite::{Connection, Result};
use std::error::Error;
use std::str::FromStr;
use async_openai::Client;

use apy_terms_scraper::apy_terms_html::{map_ids_to_scrape_to_int, read_apy_terms_html_from_file};
use apy_terms_scraper::config::Config;
use apy_terms_scraper::parse_html::StrategyFactory;
use apy_terms_scraper::institution::InstitutionName;
use apy_terms_scraper::db::{fetch_institution_names_by_account_ids, write_apy_to_db};
use apy_terms_scraper::openai::{craft_system_prompt, extract_apy_openai_call};

// TODO: provide good error messages for the most common errors
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_env()?;

    let client = Client::new();
    let conn = Connection::open(config.db_path)?;

    let ids_to_scrape = map_ids_to_scrape_to_int(config.savings_account_ids_to_scrape)?;
    let institutions_by_account_ids =
        fetch_institution_names_by_account_ids(&ids_to_scrape, &conn)?;

    for id in ids_to_scrape {
        let html_content = read_apy_terms_html_from_file(&config.apy_html_path, &id)?;
        let institution_name_str = &institutions_by_account_ids
            .get(&id)
            .ok_or("Unable to get institution name by account ID")?;
        let institution_name = InstitutionName::from_str(institution_name_str)?;
        let strategy = StrategyFactory.create(&institution_name)?;
        let terms_text = strategy.extract(&html_content)?;
        let system_prompt = craft_system_prompt(&terms_text);
        let apy = extract_apy_openai_call(&client, &system_prompt).await?;
        write_apy_to_db(&conn, id, apy)?;
    }

    Ok(())
}
