use openai_sdk::{OpenAI, types::chat::{ChatMessage, ChatCompletionRequest}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenAI::from_env()?;

    let req = ChatCompletionRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![
            ChatMessage::system("You are a helpful assistant."),
            ChatMessage::user("Say hello in 5 words."),
        ],
        ..Default::default()
    };

    let resp = client.chat_completion(req).await?;
    println!("{}", resp.first_choice_text().unwrap_or("<no text>"));
    Ok(())
}

