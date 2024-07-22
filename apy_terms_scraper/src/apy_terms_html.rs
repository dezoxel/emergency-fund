use reqwest;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::error::Error;

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
