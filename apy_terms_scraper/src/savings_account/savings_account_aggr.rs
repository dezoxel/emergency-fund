use std::error::Error;
use url::Url;

pub struct SavingsAccountAggr {
    id: i32,
    terms_and_conditions_source_url: Option<Url>,
}

impl SavingsAccountAggr {
    pub fn new(
        id: i32,
        terms_and_conditions_source_url: Option<Url>,
    ) -> Self {
        Self {
            id,
            terms_and_conditions_source_url,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn terms_and_conditions_source_url(&self) -> Option<&Url> {
        self.terms_and_conditions_source_url.as_ref()
    }

    pub fn download_terms_html(&self) -> Result<String, Box<dyn Error>> {
        let url = self.terms_and_conditions_source_url.as_ref().ok_or("Unable to download terms HTML. Source URL is empty")?;
        println!("Fetching HTML from the URL: {}", url);
        let response = reqwest::blocking::get(url.as_str())?;
        let html = response.text()?;
    
        Ok(html)
    }
}
