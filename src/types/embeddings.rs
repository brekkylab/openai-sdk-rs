use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingsRequest {
    pub model: String,
    pub input: EmbeddingInput,
    #[serde(skip_serializing_if = "Option::is_none")] pub user: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingInput {
    String(String),
    Strings(Vec<String>),
}

impl From<String> for EmbeddingInput { fn from(s: String) -> Self { EmbeddingInput::String(s) } }
impl<'a> From<&'a str> for EmbeddingInput { fn from(s: &'a str) -> Self { EmbeddingInput::String(s.to_string()) } }
impl From<Vec<String>> for EmbeddingInput { fn from(v: Vec<String>) -> Self { EmbeddingInput::Strings(v) } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingsResponse {
    pub object: String,
    pub data: Vec<EmbeddingData>,
    pub model: String,
    pub usage: Option<EmbeddingsUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingData {
    pub object: String,
    pub index: usize,
    pub embedding: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingsUsage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}

