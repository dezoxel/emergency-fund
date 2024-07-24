use std::error::Error;
use scraper::{Html, Selector};

use super::Strategy;

pub struct SoFiBankStrategy;

impl Strategy for SoFiBankStrategy {
    fn extract(&self, html_content: &str) -> Result<String, Box<dyn Error>> {
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
}
