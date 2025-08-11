use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageResponseFormat { Url, B64Json }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageGenerationRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")] pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")] pub size: Option<String>, // e.g., "1024x1024"
    #[serde(skip_serializing_if = "Option::is_none")] pub response_format: Option<ImageResponseFormat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageGenerationResponse {
    pub created: u64,
    pub data: Vec<ImageData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData {
    #[serde(skip_serializing_if = "Option::is_none")] pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub b64_json: Option<String>,
}

