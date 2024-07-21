use std::env;
use dotenv::dotenv;
use rusqlite::{Connection, Result, params_from_iter};
use std::error::Error;
use reqwest;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    
    // Connect to DB to fetch the list of URLs to scrape
    let db_path = env::var("DB_PATH")?;
    let ids_to_scrape_raw = env::var("SAVINGS_ACCOUNT_IDS_TO_SCRAPE")?;
    let apy_html_path_raw = env::var("APY_HTML_PATH")?;
    let apy_html_path = Path::new(&apy_html_path_raw).canonicalize()?;
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
    for (i, url) in urls.iter().enumerate() {
        println!("Fetching HTML from the URL: {}", url);
        let response = reqwest::blocking::get(url);
        let html_content = response?.text()?;
        let account_id = ids_to_scrape[i];
        let full_file_path = apy_html_path.join(format!("{}.html", account_id));
        println!("Storing HTML to: {}", full_file_path.display());
        let mut file = File::create(full_file_path)?;
        file.write_all(html_content.as_bytes())?;
    }

    // Parse the HTML content to extract the terms and conditions
    // Save the terms and conditions to the DB
    
    Ok(())
}