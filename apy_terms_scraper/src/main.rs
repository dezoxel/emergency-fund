use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    // Connect to DB to fetch the list of URLs to scrape
    let savings_account_ids_to_scrape_env_var_name = "SAVINGS_ACCOUNT_IDS_TO_SCRAPE";
    match env::var(savings_account_ids_to_scrape_env_var_name) {
        Ok(ids_to_scrape_raw) => {
            println!("The value of env var '{}' is '{}'", savings_account_ids_to_scrape_env_var_name, ids_to_scrape_raw);

            let ids_to_scrape: Vec<i32> = ids_to_scrape_raw
                .split(",")
                .map(|id| id.trim().parse().unwrap())
                .collect();

            println!("The savings account IDs to scrape are '{:?}'", ids_to_scrape);
        },
        Err(e) => {
            println!("Couldn't read the env var '{}'. Error: {}", savings_account_ids_to_scrape_env_var_name, e);
        }
    }
    // For each URL, fetch the HTML content
    // Parse the HTML content to extract the terms and conditions
    // Save the terms and conditions to the DB
    println!("Hello, world!");
}
