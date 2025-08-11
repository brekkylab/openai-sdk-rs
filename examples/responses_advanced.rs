use openai_sdk::{types::responses::ResponsesRequest, OpenAI};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenAI::from_env()?;

    // 示例 1: 基本文本请求带系统指令
    println!("=== Example 1: Text with Instructions ===");
    let req1 = ResponsesRequest::with_instructions(
        "gpt-4o-mini",
        "Write a haiku about programming.",
        "You are a helpful assistant who writes creative poetry. Always respond in a cheerful tone."
    );

    let resp1 = client.responses(req1).await?;
    println!(
        "Response: {}",
        resp1.output_text().unwrap_or("<no text>".into())
    );

    // 示例 2: JSON 格式响应
    println!("\n=== Example 2: JSON Response ===");
    let req2 = ResponsesRequest::json(
        "gpt-4o-mini",
        "Analyze the sentiment of this text: 'I love this new feature!'. Return the result as JSON with 'sentiment' and 'confidence' fields."
    )
    .with_max_tokens(150);

    let resp2 = client.responses(req2).await?;
    println!(
        "JSON Response: {}",
        resp2.output_text().unwrap_or("<no json>".into())
    );

    // 示例 3: 数学计算请求
    println!("\n=== Example 3: Mathematical Calculation ===");
    let mut req3 = ResponsesRequest::text(
        "gpt-4o-mini",
        "Solve this step by step: If a train travels 120 km in 2 hours, and then 180 km in 3 hours, what is its average speed for the entire journey?"
    );

    // 基本参数设置
    req3.temperature = Some(0.1); // 低温度用于数学计算
    req3.max_output_tokens = Some(500);
    req3.store = Some(false); // 不存储这次对话

    let resp3 = client.responses(req3).await?;
    println!(
        "Mathematical Response: {}",
        resp3.output_text().unwrap_or("<no text>".into())
    );

    // 示例 4: 带元数据的请求
    println!("\n=== Example 4: Request with Metadata ===");
    let mut req4 = ResponsesRequest::text(
        "gpt-4o-mini",
        "What are the main benefits of using Rust for systems programming?",
    );

    req4.metadata = Some(json!({
        "user_id": "user_123",
        "session_id": "session_456",
        "category": "programming_question"
    }));
    req4.user = Some("user_123".to_string());

    let resp4 = client.responses(req4).await?;
    println!(
        "Response with metadata: {}",
        resp4.output_text().unwrap_or("<no text>".into())
    );

    println!("\n=== Usage Information ===");
    if let Some(usage) = &resp4.usage {
        println!("Usage: {}", serde_json::to_string_pretty(usage)?);
    }

    Ok(())
}
