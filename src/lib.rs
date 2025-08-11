//! openai-sdk: A minimal async OpenAI API client for Rust.
//!
//! Features:
//! - Chat Completions: `POST /v1/chat/completions`
//! - Embeddings: `POST /v1/embeddings`
//!
//! Quickstart:
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

mod client;
mod error;
pub mod types;
pub mod sse;

pub use crate::client::OpenAI;
pub use crate::error::{ApiError, Error};
