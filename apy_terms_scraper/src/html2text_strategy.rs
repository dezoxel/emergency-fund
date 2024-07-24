use std::error::Error;

pub struct StrategyFactory;

pub trait Strategy {
    fn extract(&self, html_content: &str) -> Result<String, Box<dyn Error>>;
}
