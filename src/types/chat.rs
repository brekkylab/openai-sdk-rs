use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")] pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")] pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")] pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")] pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")] pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")] pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")] pub stop: Option<Stop>,
    #[serde(skip_serializing_if = "Option::is_none")] pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")] pub stream: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Stop {
    Single(String),
    Many(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    JsonObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")] pub name: Option<String>,
}

impl ChatMessage {
    pub fn system<T: Into<String>>(content: T) -> Self { Self { role: Role::System, content: content.into(), name: None } }
    pub fn user<T: Into<String>>(content: T) -> Self { Self { role: Role::User, content: content.into(), name: None } }
    pub fn assistant<T: Into<String>>(content: T) -> Self { Self { role: Role::Assistant, content: content.into(), name: None } }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatChoice>,
}

impl ChatCompletionResponse {
    pub fn first_choice_text(&self) -> Option<&str> {
        self.choices.get(0).map(|c| c.message.content.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    pub index: u32,
    pub message: ChatMessage,
    #[serde(skip_serializing_if = "Option::is_none")] pub finish_reason: Option<String>,
}

// Streaming chunk types for chat.completions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatChunkChoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChunkChoice {
    pub index: u32,
    pub delta: ChatDelta,
    #[serde(skip_serializing_if = "Option::is_none")] pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatDelta {
    #[serde(skip_serializing_if = "Option::is_none")] pub role: Option<Role>,
    #[serde(skip_serializing_if = "Option::is_none")] pub content: Option<String>,
}
