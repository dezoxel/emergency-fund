use rusqlite::{Connection, Result};
use std::error::Error;

use apy_terms_scraper::apy_terms_html::{
    download_apy_terms_html_by_url, map_ids_to_scrape_to_int, store_apy_terms_html_to_file,
};
use apy_terms_scraper::config::Config;
use apy_terms_scraper::db::fetch_terms_urls_by_account_ids;

// TODO: provide good error messages for the most common errors
fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_env()?;
    let ids_to_scrape = map_ids_to_scrape_to_int(config.savings_account_ids_to_scrape)?;
    let conn = Connection::open(config.db_path)?;
    let urls = fetch_terms_urls_by_account_ids(&ids_to_scrape, &conn)?;

    for (i, url) in urls.iter().enumerate() {
        let html_content = download_apy_terms_html_by_url(url)?;
        let account_id = ids_to_scrape[i];
        store_apy_terms_html_to_file(&html_content, &config.apy_html_path, &account_id)?;
    }

    Ok(())
}
