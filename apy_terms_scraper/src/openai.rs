use async_openai::{
    config::OpenAIConfig,
    types::{ChatCompletionRequestSystemMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};
use std::error::Error;

pub fn craft_system_prompt(terms_text: &str) -> String {
    let template = r#"
You will be acting as an expert in financial document analysis to extract the Annual Percentage Yield (APY) from legal texts. Your task is to identify the APY specifically applied to qualified depositors for high-yield savings accounts, excluding any APYs related to direct deposits.

Instructions:
1. Focus on identifying APY figures mentioned within the provided legal terms text below the instructions.
2. Only consider APY figures that are explicitly applied to qualified depositors.
3. Exclude any APY figures related to direct deposits.
4. Ensure the APY is for high-yield savings accounts only.
5. Output only the numerical APY value.
6. If the terms text contains only the number, return that number.
7. If you cannot find the value, return 0.

Example response: 4.5

Terms text is below:
{terms_text}
"#;

    let prompt = template.replace("{terms_text}", terms_text);
    return prompt;
}

pub async fn extract_apy_openai_call(
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

    let response = client.chat().create(request).await?;

    if let Some(first_choice) = response.choices.get(0) {
        if let Some(content) = &first_choice.message.content {
            return Ok(content.parse::<f32>()?);
        }
    }

    Err("Failed to extract APY from OpenAI response".into())
}
