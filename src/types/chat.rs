use std::collections::HashMap;

use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};

use crate::impl_builder_methods;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_tool_choice")]
    pub tool_choice: Option<ToolChoiceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
}

impl_builder_methods!(
    ChatCompletionRequest,
    temperature: f32,
    top_p: f32,
    n: u32,
    response_format: ResponseFormat,
    stream: bool,
    stop: Stop,
    max_tokens: u32,
    presence_penalty: f32,
    frequency_penalty: f32,
    tools: Vec<Tool>,
    parallel_tool_calls: bool,
    tool_choice: ToolChoiceType,
    reasoning_effort: ReasoningEffort
);

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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Tool {
    pub r#type: ToolType,
    pub function: Function,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    Function,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub parameters: FunctionParameters,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct FunctionParameters {
    #[serde(rename = "type")]
    pub schema_type: JSONSchemaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Box<JSONSchemaDefine>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum JSONSchemaType {
    Object,
    Number,
    String,
    Array,
    Null,
    Boolean,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct JSONSchemaDefine {
    #[serde(rename = "type")]
    pub schema_type: Option<JSONSchemaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Box<JSONSchemaDefine>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<JSONSchemaDefine>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ToolChoiceType {
    None,
    Auto,
    Required,
    ToolChoice { tool: Tool },
}

fn serialize_tool_choice<S>(
    value: &Option<ToolChoiceType>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(ToolChoiceType::None) => serializer.serialize_str("none"),
        Some(ToolChoiceType::Auto) => serializer.serialize_str("auto"),
        Some(ToolChoiceType::Required) => serializer.serialize_str("required"),
        Some(ToolChoiceType::ToolChoice { tool }) => {
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("type", &tool.r#type)?;
            map.serialize_entry("function", &tool.function)?;
            map.end()
        }
        None => serializer.serialize_none(),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub content: Option<MessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl ChatMessage {
    pub fn system<T: Into<String>>(content: T) -> Self {
        Self {
            role: Role::System,
            content: Some(MessageContent::Text {
                content: content.into(),
            }),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }
    pub fn user<T: Into<String>>(content: T) -> Self {
        Self {
            role: Role::User,
            content: Some(MessageContent::Text {
                content: content.into(),
            }),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }
    pub fn assistant<T: Into<String>>(content: T) -> Self {
        Self {
            role: Role::Assistant,
            content: Some(MessageContent::Text {
                content: content.into(),
            }),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }
    pub fn tool_call(tool_calls: Vec<ToolCall>) -> Self {
        Self {
            role: Role::Assistant,
            content: None,
            name: None,
            tool_calls: Some(tool_calls),
            tool_call_id: None,
        }
    }
    pub fn tool<T: Into<String>>(content: T, tool_call_id: T) -> Self {
        Self {
            role: Role::Tool,
            content: Some(MessageContent::Text {
                content: content.into(),
            }),
            name: None,
            tool_calls: None,
            tool_call_id: Some(tool_call_id.into()),
        }
    }

    pub fn new_blocks(role: Role, blocks: Vec<ContentBlock>) -> Self {
        Self {
            role,
            content: Some(MessageContent::Blocks { content: blocks }),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }
}

impl_builder_methods!(
    ChatMessage,
    content: MessageContent,
    name: String,
    tool_calls: Vec<ToolCall>,
    tool_call_id: String
);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum MessageContent {
    Text { content: String },
    Blocks { content: Vec<ContentBlock> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    Image { image_url: ImageUrlContent },
    #[serde(rename = "input_audio")]
    Audio { input_audio: AudioContent },
    #[serde(rename = "file")]
    File { file: FileContent },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageUrlContent {
    url: String,
    detail: Option<ImageDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ImageDetail {
    Auto,
    High,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AudioContent {
    data: String,
    format: AudioFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AudioFormat {
    Wav,
    Mp3,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileContent {
    file_data: Option<String>,
    file_id: Option<String>,
    filename: Option<String>,
}

impl ContentBlock {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text { text: text.into() }
    }

    pub fn image(url: impl Into<String>, detail: Option<ImageDetail>) -> Self {
        Self::Image {
            image_url: ImageUrlContent {
                url: url.into(),
                detail,
            },
        }
    }

    pub fn audio(data: impl Into<String>, format: AudioFormat) -> Self {
        Self::Audio {
            input_audio: AudioContent {
                data: data.into(),
                format,
            },
        }
    }

    pub fn file(
        file_data: Option<String>,
        file_id: Option<String>,
        filename: Option<String>,
    ) -> Self {
        Self::File {
            file: FileContent {
                file_data,
                file_id,
                filename,
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: ToolCallFunction,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatChoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    pub index: u32,
    pub message: ChatMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallDelta>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolCallDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<ToolCallFunctionDelta>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolCallFunctionDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}
