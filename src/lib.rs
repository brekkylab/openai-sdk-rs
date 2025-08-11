//! # openai-sdk
//!
//! **Unofficial**, minimal, async OpenAI API client for Rust.
//!
//! > ⚠️ **Disclaimer**: This is an **unofficial** implementation and is not affiliated with OpenAI. Use at your own discretion.
//!
//! ## Features
//!
//! - ✅ **Chat Completions**: Support for GPT-4, GPT-3.5-turbo with streaming
//! - ✅ **Embeddings**: Generate embeddings for text
//! - ✅ **Images**: Generate images with DALL-E models
//! - ✅ **Responses**: Beta responses API with advanced reasoning
//! - ✅ **Tool Calling**: Function calling with structured outputs
//! - ✅ **Streaming**: Real-time SSE streaming for chat and responses
//! - ✅ **Async/Await**: Built on tokio for high performance
//! - ✅ **Type Safety**: Comprehensive Rust types for all API endpoints
//!
//! ## Quick Start
//!
//! ```no_run
//! use openai_sdk::{OpenAI, types::chat::{ChatMessage, ChatCompletionRequest}};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = OpenAI::from_env()?; // reads OPENAI_API_KEY
//!
//! let req = ChatCompletionRequest {
//!     model: "gpt-4o-mini".to_string(),
//!     messages: vec![
//!         ChatMessage::system("You are a helpful assistant."),
//!         ChatMessage::user("Say hello in 5 words."),
//!     ],
//!     ..Default::default()
//! };
//!
//! let resp = client.chat_completion(req).await?;
//! println!("{}", resp.first_choice_text().unwrap_or_default());
//! # Ok(())
//! # }
//! ```
//!
//! ## Tool Calling Example
//!
//! ```no_run
//! use openai_sdk::{OpenAI, types::responses::{ResponsesRequest, ToolSpec}};
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = OpenAI::from_env()?;
//!
//! let tools = vec![ToolSpec {
//!     type_: "function".to_string(),
//!     name: "get_weather".to_string(),
//!     description: Some("Get current weather".to_string()),
//!     parameters: Some(json!({
//!         "type": "object",
//!         "properties": {
//!             "location": {"type": "string", "description": "City name"}
//!         },
//!         "required": ["location"]
//!     })),
//! }];
//!
//! let req = ResponsesRequest {
//!     model: "gpt-4o-2024-12-17".to_string(),
//!     input: Some(json!("What's the weather in Tokyo?")),
//!     tools: Some(tools),
//!     ..Default::default()
//! };
//!
//! let resp = client.responses(req).await?;
//! println!("{:?}", resp);
//! # Ok(())
//! # }
//! ```
//!
//! ## Environment Setup
//!
//! Set your OpenAI API key:
//!
//! ```bash
//! export OPENAI_API_KEY="your-api-key-here"
//! ```

mod client;
mod error;
pub mod types;
pub mod sse;

pub use crate::client::OpenAI;
pub use crate::error::{ApiError, Error};
