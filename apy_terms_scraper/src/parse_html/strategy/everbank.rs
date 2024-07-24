use std::error::Error;
use scraper::{Html, Selector};

use super::Strategy;

pub struct EverBankStrategy;

impl Strategy for EverBankStrategy {
    fn extract(&self, html_content: &str) -> Result<String, Box<dyn Error>> {
        println!("Parsing HTML content...");

        let document = Html::parse_document(&html_content);
        let terms_selector = Selector::parse(r#"[data-rate-prop="apy"][data-parse-entity="rate"]"#)?;
        let mut terms_html = document.select(&terms_selector);
        let terms_text = terms_html
            .next()
            .unwrap()
            .text()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("");

        Ok(terms_text)
    }
}
