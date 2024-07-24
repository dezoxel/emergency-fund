use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::error::Error;

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
