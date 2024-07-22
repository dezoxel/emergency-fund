use reqwest;
use std::path::Path;
use std::fs::File;
use std::io::{Write, Read};
use std::error::Error;
use scraper::{Html, Selector};

pub fn map_ids_to_scrape_to_int(ids_to_scrape_raw: String) -> Result<Vec<i32>, Box<dyn Error>> {
    let ids_to_scrape: Vec<i32> = ids_to_scrape_raw
        .split(',')
        .map(|id| id.trim().parse().unwrap())
        .collect();

    Ok(ids_to_scrape)
}

pub fn download_apy_terms_html_by_url(url: &str) -> Result<String, Box<dyn Error>> {
    println!("Fetching HTML from the URL: {}", url);
    let response = reqwest::blocking::get(url)?;
    let html_content = response.text()?;

    Ok(html_content)
}

pub fn store_apy_terms_html_to_file(html_content: &str, apy_html_path: &str, file_name: &i32) -> Result<(), Box<dyn Error>> {
    let base_dir = Path::new(&apy_html_path).canonicalize()?;
    let full_file_path = base_dir.join(format!("{}.html", file_name));
    println!("Storing HTML to: {}", full_file_path.display());
    let mut file = File::create(full_file_path)?;
    file.write_all(html_content.as_bytes())?;

    Ok(())
}

// TODO: consider creating a type like APYHtmlFile to store the path and the content
pub fn read_apy_terms_html_from_file(apy_html_path: &str, file_name: &i32) -> Result<String, Box<dyn Error>> {
    let base_dir = Path::new(&apy_html_path).canonicalize()?;
    let full_file_path = base_dir.join(format!("{}.html", file_name));
    println!("Reading HTML from: {}", full_file_path.display());
    let mut file = File::open(full_file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

// TODO: this works only for a specific case, consider making it generic
pub fn select_terms_text(html_content: &str) -> Result<String, Box<dyn Error>> {
    println!("Parsing HTML content...");

    let document = Html::parse_document(&html_content);
    // TODO: extract selector to config or constant
    let terms_selector = Selector::parse(".subfooter__legal")?;
    let mut terms_html = document.select(&terms_selector);
    let terms_text = terms_html
        .next()
        .unwrap()
        .text()
        // TODO: extract to constant or config
        .filter(|s| !s.trim_start().starts_with("(func"))
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("");

    Ok(terms_text)
}