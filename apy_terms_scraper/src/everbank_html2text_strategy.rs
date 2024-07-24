use std::error::Error;

use crate::html2text_strategy::Strategy;

pub struct EverBankStrategy;

impl Strategy for EverBankStrategy {
    fn extract(&self, _html_content: &str) -> Result<String, Box<dyn Error>> {
        Ok("EverBankStrategy".to_string())
    }
}
