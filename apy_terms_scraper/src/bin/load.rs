use std::error::Error;

use apy_terms_scraper::apy_terms_html::{
    map_ids_to_scrape_to_int, read_apy_terms_html_from_file, select_terms_text,
};
use apy_terms_scraper::config::Config;

// TODO: provide good error messages for the most common errors
fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_env()?;

    let ids_to_scrape = map_ids_to_scrape_to_int(config.savings_account_ids_to_scrape)?;

    for id in ids_to_scrape {
        let html_content = read_apy_terms_html_from_file(&config.apy_html_path, &id)?;
        let terms_text = select_terms_text(&html_content)?;
        println!("Terms for account ID {}: {:?}", id, terms_text);
    }

    Ok(())
}
