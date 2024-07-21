use std::env;
use dotenv::dotenv;
use rusqlite::{Connection, Result, params_from_iter};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    
    // Connect to DB to fetch the list of URLs to scrape
    let db_path = env::var("DB_PATH")?;
    let ids_to_scrape_raw = env::var("SAVINGS_ACCOUNT_IDS_TO_SCRAPE")?;
    let ids_to_scrape: Vec<i32> = ids_to_scrape_raw
        .split(',')
        .map(|id| id.trim().parse().unwrap())
        .collect();

    let conn = Connection::open(db_path)?;

    let query = format!(
        "SELECT terms_and_conditions_source_url FROM savings_accounts WHERE id IN ({})",
        ids_to_scrape.iter().map(|_| "?").collect::<Vec<_>>().join(",")
    );

    let mut stmt = conn.prepare(&query)?;
    let url_iter = stmt.query_map(params_from_iter(ids_to_scrape.iter()), |row| row.get::<_, Option<String>>(0))?;
    let urls: Vec<String> = url_iter.filter_map(Result::ok).filter_map(|url| url).collect();

    println!("The URLs are: {:?}", urls);

    // For each URL, fetch the HTML content
    // Parse the HTML content to extract the terms and conditions
    // Save the terms and conditions to the DB
    
    Ok(())
}