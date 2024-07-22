use rusqlite::{params_from_iter, Connection, Result};
use std::error::Error;

use apy_terms_scraper::apy_terms_html::{
    download_apy_terms_html_by_url, map_ids_to_scrape_to_int, store_apy_terms_html_to_file,
};
use apy_terms_scraper::config::Config;

fn fetch_terms_urls_by_account_ids(
    ids: &Vec<i32>,
    conn: &Connection,
) -> Result<Vec<String>, Box<dyn Error>> {
    let query = format!(
        "SELECT terms_and_conditions_source_url FROM savings_accounts WHERE id IN ({})",
        ids.iter().map(|_| "?").collect::<Vec<_>>().join(",")
    );

    let mut stmt = conn.prepare(&query)?;
    let url_iter = stmt.query_map(params_from_iter(ids.iter()), |row| {
        row.get::<_, Option<String>>(0)
    })?;
    let urls: Vec<String> = url_iter
        .filter_map(Result::ok)
        .filter_map(|url| url)
        .collect();

    Ok(urls)
}

// TODO: provide good error messages for the most common errors
fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_env()?;

    // Connect to DB to fetch the list of URLs to scrape
    let ids_to_scrape = map_ids_to_scrape_to_int(config.savings_account_ids_to_scrape)?;

    let conn = Connection::open(config.db_path)?;

    let urls = fetch_terms_urls_by_account_ids(&ids_to_scrape, &conn)?;

    // For each URL, fetch the HTML content
    for (i, url) in urls.iter().enumerate() {
        let html_content = download_apy_terms_html_by_url(url)?;
        let account_id = ids_to_scrape[i];
        store_apy_terms_html_to_file(&html_content, &config.apy_html_path, &account_id)?;
    }

    // Parse the HTML content to extract the terms and conditions
    // Save the terms and conditions to the DB

    Ok(())
}
