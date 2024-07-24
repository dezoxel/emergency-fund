use std::path::Path;
use std::fs::File;
use std::io::{Write, Read};
use std::error::Error;

pub fn map_ids_to_scrape_to_int(ids_to_scrape_raw: String) -> Result<Vec<i32>, Box<dyn Error>> {
    let ids_to_scrape: Vec<i32> = ids_to_scrape_raw
        .split(',')
        .map(|id| id.trim().parse().unwrap())
        .collect();

    Ok(ids_to_scrape)
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
