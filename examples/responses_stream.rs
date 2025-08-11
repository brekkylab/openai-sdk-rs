use futures_util::TryStreamExt;
use openai_sdk_rs::{
    types::responses::{ResponsesRequest, StreamOptions},
    OpenAI,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting responses_stream example...");

    let client = OpenAI::from_env()?;
    println!("Client created successfully");

    let mut req =
        ResponsesRequest::text("gpt-4o-mini", "Stream a short response about rustaceans.");
    // Try without include_usage parameter
    req.stream_options = Some(StreamOptions {
        include_usage: None,
    });

    println!("Making request to /v1/responses...");
    let mut stream = client.responses_stream(req).await?;
    println!("Stream created, waiting for events...");

    let mut event_count = 0;
    while let Some(event) = stream.try_next().await? {
        event_count += 1;

        // Only log non-delta events to reduce noise
        if !event.type_.contains("delta") {
            println!("Received event #{}: {:?}", event_count, event.type_);
        }

        if let Some(delta) = &event.output_text {
            // The event may include text delta depending on server shape
            print!("{}", delta);
        } else if let Some(delta_value) = &event.delta {
            // Extract text from delta field
            if let Some(text) = delta_value.as_str() {
                print!("{}", text);
            } else if let Some(text) = delta_value.get("text").and_then(|v| v.as_str()) {
                print!("{}", text);
            } else if let Some(text) = delta_value.get("output_text").and_then(|v| v.as_str()) {
                print!("{}", text);
            }
        } else if let Some(response) = &event.response {
            // Try to extract text from response object
            if let Some(text) = response.get("output").and_then(|v| v.as_str()) {
                print!("{}", text);
            } else if let Some(delta_obj) = response.get("delta") {
                if let Some(text) = delta_obj.get("text").and_then(|v| v.as_str()) {
                    print!("{}", text);
                }
            }
        }
    }

    println!();
    println!("Stream ended. Total events received: {}", event_count);
    Ok(())
}
