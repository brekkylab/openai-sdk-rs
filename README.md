# openai-sdk-rs

[![Crates.io](https://img.shields.io/crates/v/openai-sdk-rs.svg)](https://crates.io/crates/openai-sdk-rs)
[![Documentation](https://docs.rs/openai-sdk-rs/badge.svg)](https://docs.rs/openai-sdk-rs)
[![License](https://img.shields.io/crates/l/openai-sdk-rs.svg)](https://github.com/neeboo/openai-sdk-rs#license)

**Unofficial**, minimal, async OpenAI API client for Rust.

> ⚠️ **Disclaimer**: This is an **unofficial** implementation and is not affiliated with OpenAI. Use at your own discretion.

## ✨ Features

What it covers:
- Chat Completions: `POST /v1/chat/completions`
- Embeddings: `POST /v1/embeddings`
- Streaming for Chat Completions (SSE)
- **Responses API (+ streaming)** - Full support with tool calling
- Images generation
- Files list + upload (multipart)
- **Advanced configurations** - Temperature, max tokens, reasoning, etc.

Not a full mirror of the API yet — intentionally small and focused.

## 🚀 Quick Start

## 📚 Examples

Check out the `examples/` directory for comprehensive usage examples:

- `cargo run --example chat` - Basic chat completion
- `cargo run --example chat_stream` - Streaming chat
- `cargo run --example responses` - Basic responses API
- `cargo run --example responses_stream` - Streaming responses
- `cargo run --example responses_tool_call` - Tool calling with functions
- `cargo run --example responses_advanced` - Advanced parameters and configurations
- `cargo run --example images` - Image generation

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
openai-sdk-rs = { path = "." }
```

Or once published:

```toml
openai-sdk-rs = "0.1"
```

## Configure

Copy `.env.example` to `.env` and fill your values, or export env vars directly.

```bash
export OPENAI_API_KEY=sk-...
# optional scoping
export OPENAI_ORG_ID=org_...
export OPENAI_PROJECT_ID=proj_...
```

To use a proxy-compatible base URL:

```bash
export OPENAI_BASE_URL=https://api.openai.com/v1
```

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
openai-sdk-rs = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

Or install via cargo:

```bash
cargo add openai-sdk-rs
```

## 🚀 Quick Start

You'll need an OpenAI API key. Set it as an environment variable:

```bash
export OPENAI_API_KEY="your-api-key-here"
```

## 🔧 Usage Examples

Chat:

```rust
use openai_sdk_rs::{OpenAI, types::chat::{ChatMessage, ChatCompletionRequest}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenAI::from_env()?;

    let req = ChatCompletionRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![
            ChatMessage::system("You are a helpful assistant."),
            ChatMessage::user("Write a haiku about Rust."),
        ],
        ..Default::default()
    };

    let resp = client.chat_completion(req).await?;
    println!("{}", resp.first_choice_text().unwrap_or("<no text>"));
    Ok(())
}
```

Embeddings:

```rust
use openai_sdk_rs::{OpenAI, types::embeddings::{EmbeddingsRequest, EmbeddingInput}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenAI::from_env()?;
    let req = EmbeddingsRequest { model: "text-embedding-3-small".into(), input: EmbeddingInput::from("hello world"), user: None };
    let resp = client.embeddings(req).await?;
    println!("{} vectors", resp.data.len());
    Ok(())
}
```

Streaming chat:

```rust
use openai_sdk_rs::{OpenAI, types::chat::{ChatMessage, ChatCompletionRequest}};
use futures_util::TryStreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenAI::from_env()?;
    let req = ChatCompletionRequest { model: "gpt-4o-mini".into(), messages: vec![ChatMessage::user("Stream a short line.")], ..Default::default() };
    let mut stream = client.chat_completion_stream(req).await?;
    while let Some(chunk) = stream.try_next().await? {
        if let Some(text) = chunk.choices.get(0).and_then(|c| c.delta.content.as_deref()) {
            print!("{}", text);
        }
    }
    println!();
    Ok(())
}
```

Responses API:

```rust
use openai_sdk_rs::{OpenAI, types::responses::ResponsesRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenAI::from_env()?;
    let resp = client.responses(ResponsesRequest::text("gpt-4o-mini", "One sentence about Rust.")).await?;
    println!("{}", resp.output_text().unwrap_or("<no text>".into()));
    Ok(())
}
```

Streaming Responses:

```rust
use openai_sdk_rs::{OpenAI, types::responses::{ResponsesRequest, StreamOptions}};
use futures_util::TryStreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenAI::from_env()?;
    let mut req = ResponsesRequest::text("gpt-4o-mini", "Stream a short fact about whales.");
    req.stream_options = Some(StreamOptions { include_usage: Some(true) });
    let mut stream = client.responses_stream(req).await?;
    while let Some(event) = stream.try_next().await? {
        if let Some(text) = event.output_text {
            print!("{}", text);
        }
    }
    println!();
    Ok(())
}
```
Aggregated streaming helpers:

```rust
use openai_sdk_rs::{OpenAI, types::chat::{ChatMessage, ChatCompletionRequest}, types::responses::ResponsesRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenAI::from_env()?;

    // Chat text via streaming aggregation
    let chat_text = client.chat_completion_stream_text(ChatCompletionRequest {
        model: "gpt-4o-mini".into(),
        messages: vec![ChatMessage::user("Say something short.")],
        ..Default::default()
    }).await?;
    println!("chat: {}", chat_text);

    // Responses text via streaming aggregation
    let resp_text = client.responses_stream_text(ResponsesRequest::text("gpt-4o-mini", "Stream one line.")).await?;
    println!("responses: {}", resp_text);

    Ok(())
}
```

SSE helper utilities:

```rust
use openai_sdk_rs::sse::{extract_data_lines_from_bytes, extract_json_values_from_bytes, extract_data_lines_from_str, try_extract_json_values_from_str};

let raw = b"data: {\"a\":1}\n\n:data comment\n\ndata: [DONE]\n";
let lines = extract_data_lines_from_bytes(raw);
assert_eq!(lines, vec!["{\"a\":1}".to_string()]);
let jsons = extract_json_values_from_bytes(raw);
assert_eq!(jsons[0]["a"], 1);

// String-based variants and Result-returning versions
let text = "data: {\"a\":1}\n\n";
let lines_str = extract_data_lines_from_str(text);
assert_eq!(lines_str, vec!["{\"a\":1}"]);
let jsons_str = try_extract_json_values_from_str(text).unwrap();
assert_eq!(jsons_str[0]["a"], 1);
```

## Goals

- Keep a clean, small surface area
- Prioritize reliability and clear errors
- Include light retries with exponential backoff for 429/5xx/timeouts
- Be easy to extend for more endpoints later

## Images:

```rust
use openai_sdk_rs::{OpenAI, types::images::{ImageGenerationRequest, ImageResponseFormat}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenAI::from_env()?;
    let req = ImageGenerationRequest { 
        model: "dall-e-3".into(), 
        prompt: "A tiny Rust crab".into(), 
        n: Some(1), 
        size: Some("1024x1024".into()), 
        response_format: Some(ImageResponseFormat::B64Json) 
    };
    let resp = client.images_generate(req).await?;
    println!("variants: {}", resp.data.len());
    Ok(())
}
```

## Tool calling (Responses)

Supply tools via `ResponsesRequest.tools` with JSON Schema:

```rust
use openai_sdk_rs::OpenAI;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenAI::from_env()?;
    let files = client.files_list().await?;
    println!("{} files", files.data.len());

    // Download and delete
    if let Some(f) = files.data.first() {
        let bytes = client.files_download(&f.id).await?;
        println!("downloaded {} bytes from {}", bytes.len(), f.filename);
        let del = client.files_delete(&f.id).await?;
        println!("deleted {}: {}", del.id, del.deleted);
    }
    Ok(())
}
```

```rust
use openai_sdk_rs::types::responses::{ResponsesRequest, ToolSpec};
use serde_json::json;

let req = ResponsesRequest {
    model: "gpt-4o-mini".into(),
    input: Some(json!("What's the weather in SF?")),
    tools: Some(vec![ToolSpec {
        type_: "function".to_string(),
        name: "get_weather".to_string(),
        description: Some("Get weather by city".to_string()),
        parameters: Some(json!({
            "type": "object",
            "properties": {"city": {"type": "string"}},
            "required": ["city"],
        })),
    }]),
    ..Default::default()
};

// After calling `client.responses(req).await?`:
// let resp: ResponsesResponse = ...;
// for call in resp.function_calls() {
//     println!("tool: {} args: {}", call.name, call.arguments);
// }

// For streaming events:
// let mut stream = client.responses_stream(req).await?;
// while let Some(ev) = stream.try_next().await? {
//     for call in ev.function_calls() {
//         println!("stream tool: {} args: {}", call.name, call.arguments);
//     }
// }
```

## Builder options

- `timeout(Duration)` set request timeout
- `max_retries(u32)` and `retry_base_delay(Duration)` configure retries
- `proxy(url)` set an HTTP(S) proxy for all requests

## Custom reqwest Client

If you need full control over HTTP behavior (proxies, pools, TLS, UA), inject your own `reqwest::Client`:

```rust
use std::time::Duration;
use openai_sdk_rs::{OpenAI, types::chat::{ChatMessage, ChatCompletionRequest}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let http = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("my-app/0.1")
        .build()?;

    let api_key = std::env::var("OPENAI_API_KEY")?;
    let oai = OpenAI::with_http_client(http, api_key)?;

    let resp = oai.chat_completion(ChatCompletionRequest {
        model: "gpt-4o-mini".into(),
        messages: vec![ChatMessage::user("Hello from custom client!")],
        ..Default::default()
    }).await?;
    println!("{}", resp.first_choice_text().unwrap_or("<no text>"));
    Ok(())
}
```

Or via the builder:

```rust
let http = reqwest::Client::builder().build()?;
let oai = OpenAI::builder()
    .http_client(http)
    .api_key(std::env::var("OPENAI_API_KEY")?)
    .build()?;
```

Note: when injecting a client, builder options like `timeout`, `proxy`, and `user_agent` are not applied; configure them on your `reqwest::Client`.

## License

MIT or Apache-2.0, at your option.