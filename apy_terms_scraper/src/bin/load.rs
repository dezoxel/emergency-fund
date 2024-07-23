use async_openai::config::OpenAIConfig;
use async_openai::{
    types::{ChatCompletionRequestSystemMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};
use std::error::Error;
use chrono::Utc;
use rusqlite::{params, Connection, Result};

use apy_terms_scraper::apy_terms_html::{
    map_ids_to_scrape_to_int, read_apy_terms_html_from_file, select_terms_text,
};
use apy_terms_scraper::config::Config;

fn craft_system_prompt(terms_text: &str) -> String {
    let template = r#"
You will be acting as an expert in financial document analysis to extract the Annual Percentage Yield (APY) from legal texts. Your task is to identify the APY specifically applied to qualified depositors for high-yield savings accounts, excluding any APYs related to direct deposits.

Instructions:
1. Focus on identifying APY figures mentioned within the provided legal text enclosed in '<terms-text>' tags.
2. Only consider APY figures that are explicitly applied to qualified depositors.
3. Exclude any APY figures related to direct deposits.
4. Ensure the APY is for high-yield savings accounts only.
5. Output only the numerical APY value.

Example response: 4.5

<terms-text>
{terms_text}
</terms-text>
"#;

    let prompt = template.replace("{terms_text}", terms_text);
    return prompt;
}

async fn extract_apy_openai_call(
    client: &Client<OpenAIConfig>,
    system_prompt: &str,
) -> Result<f32, Box<dyn Error>> {
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-4o-mini")
        .messages([ChatCompletionRequestSystemMessageArgs::default()
            .content(system_prompt)
            .build()?
            .into()])
        .build()?;

    println!("{}", serde_json::to_string(&request).unwrap());

    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");
    for choice in &response.choices {
        println!(
            "{}: Role: {}  Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        );
    }

    if let Some(first_choice) = response.choices.get(0) {
        if let Some(content) = &first_choice.message.content {
            println!("Content: {:?}", content);
            return Ok(content.parse::<f32>()?);
        }
    }

    Err("Failed to extract APY from OpenAI response".into())
}

fn write_apy_to_db(conn: &Connection, account_id: i32, apy: f32) -> Result<(), Box<dyn Error>> {
    println!("Writing APY to DB... Account ID: {}, APY: {}", account_id, apy);
    let query = "INSERT INTO savings_accounts_apy_history (account_id, apy, compound_frequency, effective_date) VALUES (?1, ?2, ?3, ?4)";
    let compound_frequency = 365;
    let effective_date = Utc::now().format("%Y-%m-%d").to_string();
    let params = params![account_id, apy, compound_frequency, effective_date];
    conn.execute(query, params)?;
    println!("APY is written to the database");

    Ok(())
}

// TODO: provide good error messages for the most common errors
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_env()?;

    let client = Client::new();
    let conn = Connection::open(config.db_path)?;

    let ids_to_scrape = map_ids_to_scrape_to_int(config.savings_account_ids_to_scrape)?;

    for id in ids_to_scrape {
        let html_content = read_apy_terms_html_from_file(&config.apy_html_path, &id)?;
        let terms_text = select_terms_text(&html_content)?;
        let system_prompt = craft_system_prompt(&terms_text);
        let apy = extract_apy_openai_call(&client, &system_prompt).await?;
        write_apy_to_db(&conn, id, apy)?;
    }

    Ok(())
}
