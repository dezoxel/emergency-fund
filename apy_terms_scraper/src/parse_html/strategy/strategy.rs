use std::error::Error;

pub trait Strategy {
    fn extract(&self, html_content: &str) -> Result<String, Box<dyn Error>>;
}
