use apy_terms_scraper::apy_terms_html::{
    map_ids_to_scrape_to_int, read_apy_terms_html_from_file, select_terms_text,
};
use apy_terms_scraper::config::Config;
use async_openai::{
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use std::error::Error;

// TODO: provide good error messages for the most common errors
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_env()?;

    let ids_to_scrape = map_ids_to_scrape_to_int(config.savings_account_ids_to_scrape)?;

    for id in ids_to_scrape {
        let html_content = read_apy_terms_html_from_file(&config.apy_html_path, &id)?;
        let terms_text = select_terms_text(&html_content)?;
        println!("Terms for account ID {}: {:?}", id, terms_text);
    }

    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-4o-mini")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("who is the president of the US? just two words")
                .build()?
                .into(),
        ])
        .build()?;

    println!("{}", serde_json::to_string(&request).unwrap());

    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");
    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        );
    }

    Ok(())
}
