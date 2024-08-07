use log::info;
use std::error::Error;
use url::Url;
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use async_openai::{Client, config::OpenAIConfig};

use crate::institution::InstitutionName;
use crate::parse_html::StrategyFactory;
use crate::openai::{craft_system_prompt, extract_apy_openai_call};

pub struct SavingsAccountAggr {
    id: i32,
    terms_and_conditions_source_url: Option<Url>,
    institution_name: InstitutionName,
}

impl SavingsAccountAggr {
    pub fn new(
        id: i32,
        terms_and_conditions_source_url: Option<Url>,
        institution_name: InstitutionName,
    ) -> Self {
        Self {
            id,
            terms_and_conditions_source_url,
            institution_name,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn terms_and_conditions_source_url(&self) -> Option<&Url> {
        self.terms_and_conditions_source_url.as_ref()
    }

    pub fn institution_name(&self) -> &InstitutionName {
        &self.institution_name
    }

    pub fn download_terms_html(&self) -> Result<String, Box<dyn Error>> {
        let url = self.terms_and_conditions_source_url.as_ref().ok_or("Unable to download terms HTML. Source URL is empty")?;
        info!("Fetching HTML from the URL: {}", url);
        let response = reqwest::blocking::get(url.as_str())?;
        let html = response.text()?;
    
        Ok(html)
    }

    pub fn write_terms_html_to_file(&self, html_dir: &str, html: &str) -> Result<(), Box<dyn Error>> {
        let file_name = self.id;
        let base_dir = Path::new(&html_dir).canonicalize()?;
        let full_file_path = base_dir.join(format!("{}.html", file_name));
        info!("Storing HTML to: {}", full_file_path.display());
        let mut file = File::create(full_file_path)?;
        file.write_all(html.as_bytes())?;
    
        Ok(())
    }

    pub fn read_terms_html_from_file(&self, html_dir: &str) -> Result<String, Box<dyn Error>> {
        let file_name = self.id;
        let base_dir = Path::new(&html_dir).canonicalize()?;
        let full_file_path = base_dir.join(format!("{}.html", file_name));
        info!("Reading HTML from: {}", full_file_path.display());
        let mut file = File::open(full_file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
    
        Ok(contents)
    }

    pub fn extract_terms_text_from_html(&self, html: &str) -> Result<String, Box<dyn Error>> {
        let strategy = StrategyFactory.create(&self.institution_name)?;
        let terms_text = strategy.extract(&html)?;
        Ok(terms_text)
    }

    pub async fn extract_apy_from_terms_text(&self, client: &Client<OpenAIConfig>, terms_text: &str) -> Result<f64, Box<dyn Error>> {
        let system_prompt = craft_system_prompt(&terms_text);
        let apy = extract_apy_openai_call(&client, &system_prompt).await?;
        Ok(apy)
    }
}
