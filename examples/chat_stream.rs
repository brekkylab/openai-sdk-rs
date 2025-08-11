use futures_util::TryStreamExt;
use openai_sdk_rs::{
    types::chat::{ChatCompletionRequest, ChatMessage},
    OpenAI,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenAI::from_env()?;
    println!("{:?}", client);
    let req = ChatCompletionRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![
            ChatMessage::system("You are a helpful assistant."),
            ChatMessage::user("Stream one short sentence about Rust."),
        ],
        ..Default::default()
    };

    let mut stream = client.chat_completion_stream(req).await?;
    while let Some(chunk) = stream.try_next().await? {
        if let Some(text) = chunk
            .choices
            .first()
            .and_then(|c| c.delta.content.as_deref())
        {
            print!("{}", text);
        }
    }
    println!();
    Ok(())
}
