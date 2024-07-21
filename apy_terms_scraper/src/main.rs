use std::env;
use dotenv::dotenv;
use rusqlite::{Connection, Result, params_from_iter};
use std::error::Error;
use reqwest;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn fetch_terms_urls_by_account_ids(ids: &Vec<i32>, conn: &Connection) -> Result<Vec<String>, Box<dyn Error>> {
    let query = format!(
        "SELECT terms_and_conditions_source_url FROM savings_accounts WHERE id IN ({})",
        ids.iter().map(|_| "?").collect::<Vec<_>>().join(",")
    );

    let mut stmt = conn.prepare(&query)?;
    let url_iter = stmt.query_map(params_from_iter(ids.iter()), |row| row.get::<_, Option<String>>(0))?;
    let urls: Vec<String> = url_iter.filter_map(Result::ok).filter_map(|url| url).collect();

    Ok(urls)
}

fn read_ids_to_scrape_from_env() -> Result<Vec<i32>, Box<dyn Error>> {
    let ids_to_scrape_raw = env::var("SAVINGS_ACCOUNT_IDS_TO_SCRAPE")?;
    let ids_to_scrape: Vec<i32> = ids_to_scrape_raw
        .split(',')
        .map(|id| id.trim().parse().unwrap())
        .collect();

    Ok(ids_to_scrape)
}

fn download_apy_terms_html_by_url(url: &str) -> Result<String, Box<dyn Error>> {
    println!("Fetching HTML from the URL: {}", url);
    let response = reqwest::blocking::get(url)?;
    let html_content = response.text()?;

    Ok(html_content)
}

fn store_apy_terms_html_to_file(html_content: &str, base_dir: &Path, file_name: &i32) -> Result<(), Box<dyn Error>> {
    let full_file_path = base_dir.join(format!("{}.html", file_name));
    println!("Storing HTML to: {}", full_file_path.display());
    let mut file = File::create(full_file_path)?;
    file.write_all(html_content.as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    
    // Connect to DB to fetch the list of URLs to scrape
    let db_path = env::var("DB_PATH")?;
    let apy_html_path_raw = env::var("APY_HTML_PATH")?;
    let apy_html_path = Path::new(&apy_html_path_raw).canonicalize()?;
    let ids_to_scrape = read_ids_to_scrape_from_env()?;

    let conn = Connection::open(db_path)?;

    let urls = fetch_terms_urls_by_account_ids(&ids_to_scrape, &conn)?;

    println!("The URLs are: {:?}", urls);

    // For each URL, fetch the HTML content
    for (i, url) in urls.iter().enumerate() {
        let html_content = download_apy_terms_html_by_url(url)?;
        let account_id = ids_to_scrape[i];
        store_apy_terms_html_to_file(&html_content, &apy_html_path, &account_id)?;
    }

    // Parse the HTML content to extract the terms and conditions
    // Save the terms and conditions to the DB
    
    Ok(())
}
