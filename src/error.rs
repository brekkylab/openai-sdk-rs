use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorBody {
    pub message: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub param: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorEnvelope {
    pub error: ApiErrorBody,
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("missing API key; set OPENAI_API_KEY or pass explicitly")] 
    MissingApiKey,

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("api error: {0}")]
    Api(#[from] ApiError),

    #[error("url parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("unexpected status {status}: {body}")]
    UnexpectedStatus { status: u16, body: String },
}

#[derive(Debug, Clone, ThisError, Serialize, Deserialize)]
#[error("{message}")]
pub struct ApiError {
    pub message: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub param: Option<String>,
    pub code: Option<String>,
    #[serde(skip)]
    pub status: Option<u16>,
}

impl From<ApiErrorEnvelope> for ApiError {
    fn from(env: ApiErrorEnvelope) -> Self {
        ApiError {
            message: env.error.message,
            type_: env.error.type_,
            param: env.error.param,
            code: env.error.code,
            status: None,
        }
    }
}
