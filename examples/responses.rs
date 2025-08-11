use openai_sdk::{types::responses::ResponsesRequest, OpenAI};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenAI::from_env()?;
    let req = ResponsesRequest::text("gpt-4o-mini", "Write a two-line poem about oceans.");
    let resp = client.responses(req).await?;
    println!("{}", resp.output_text().unwrap_or("<no text>".into()));
    Ok(())
}
