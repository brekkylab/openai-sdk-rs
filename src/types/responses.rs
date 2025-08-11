use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResponsesRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolSpec>>, // tool calling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<serde_json::Value>, // leave generic for now
    // 新增参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>, // System instructions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>, // Request metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>, // Whether to enable parallel tool calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextConfig>, // Text configuration (replaces response_format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>, // Random seed for deterministic outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>, // Whether to store the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>, // Reasoning configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>, // Number of most likely tokens to return at each position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>, // Whether to return log probabilities
}

impl ResponsesRequest {
    pub fn text<T: Into<String>>(model: T, input: T) -> Self {
        Self {
            model: model.into(),
            input: Some(serde_json::Value::String(input.into())),
            ..Default::default()
        }
    }

    // 创建带有系统指令的请求
    pub fn with_instructions<T: Into<String>>(model: T, input: T, instructions: T) -> Self {
        Self {
            model: model.into(),
            input: Some(serde_json::Value::String(input.into())),
            instructions: Some(instructions.into()),
            ..Default::default()
        }
    }

    // 创建需要 JSON 响应的请求
    pub fn json<T: Into<String>>(model: T, input: T) -> Self {
        Self {
            model: model.into(),
            input: Some(serde_json::Value::String(input.into())),
            text: Some(TextConfig {
                format: Some(TextFormat::JsonObject),
                verbosity: None,
            }),
            ..Default::default()
        }
    }

    // 创建带有特定温度的请求
    pub fn with_temperature<T: Into<String>>(model: T, input: T, temperature: f32) -> Self {
        Self {
            model: model.into(),
            input: Some(serde_json::Value::String(input.into())),
            temperature: Some(temperature),
            ..Default::default()
        }
    }

    // 设置最大输出 tokens
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_output_tokens = Some(max_tokens);
        self
    }

    // 设置种子以获得确定性输出
    pub fn with_seed(mut self, seed: i32) -> Self {
        self.seed = Some(seed);
        self
    }

    // 启用并行工具调用
    pub fn with_parallel_tools(mut self, parallel: bool) -> Self {
        self.parallel_tool_calls = Some(parallel);
        self
    }

    // 添加工具
    pub fn with_tools(mut self, tools: Vec<ToolSpec>) -> Self {
        self.tools = Some(tools);
        self
    }

    // 设置工具选择策略
    pub fn with_tool_choice<T: serde::Serialize>(mut self, choice: T) -> Self {
        self.tool_choice = Some(serde_json::to_value(choice).unwrap_or(serde_json::Value::Null));
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StreamOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}

// Response format specification (legacy, kept for compatibility)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    JsonObject,
    JsonSchema { json_schema: JsonSchemaSpec },
}

// Text configuration for Responses API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<TextFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<String>, // "low", "medium", "high"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TextFormat {
    Text,
    JsonObject,
    JsonSchema { json_schema: JsonSchemaSpec },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonSchemaSpec {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub schema: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

// Reasoning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>, // "low", "medium", "high"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsesResponse {
    pub id: String,
    pub object: String,
    #[serde(default)]
    pub created: Option<u64>,
    pub model: String,
    // Keep generic for forward-compat
    #[serde(default)]
    pub output: Option<serde_json::Value>,
    #[serde(default)]
    pub usage: Option<serde_json::Value>,
}

impl ResponsesResponse {
    // Best-effort extraction of plain text from common shapes.
    pub fn output_text(&self) -> Option<String> {
        let v = self.output.as_ref()?;
        // If it's already a string
        if let Some(s) = v.as_str() {
            return Some(s.to_string());
        }
        // Try known shapes (message/content/text)
        // Many SDKs expose a convenience text; we attempt similar.
        // This is intentionally lenient.
        let mut buf = String::new();
        collect_text(v, &mut buf);
        if buf.is_empty() {
            None
        } else {
            Some(buf)
        }
    }

    pub fn output_json(&self) -> Option<serde_json::Value> {
        let text = self.output_text()?;
        serde_json::from_str(&text).ok()
    }
}

fn collect_text(v: &serde_json::Value, out: &mut String) {
    match v {
        serde_json::Value::String(s) => {
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(s);
        }
        serde_json::Value::Array(arr) => {
            for item in arr {
                collect_text(item, out);
            }
        }
        serde_json::Value::Object(map) => {
            for (k, val) in map {
                if k.contains("text") || k == "content" {
                    collect_text(val, out);
                }
            }
        }
        _ => {}
    }
}

// Streaming events for Responses API; keep generic but typed by 'type'
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseStreamEvent {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub response: Option<serde_json::Value>,
    #[serde(default)]
    pub output_text: Option<String>,
    #[serde(default)]
    pub delta: Option<serde_json::Value>,
    #[serde(default)]
    pub message: Option<serde_json::Value>,
    #[serde(default)]
    pub usage: Option<serde_json::Value>,
}

// Tool specification for function calling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSpec {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>, // JSON Schema
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: serde_json::Value,
}

impl super::responses::ResponsesResponse {
    pub fn function_calls(&self) -> Vec<FunctionCall> {
        let mut out = Vec::new();
        if let Some(v) = &self.output {
            collect_function_calls(v, &mut out);
        }
        out
    }
}

impl super::responses::ResponseStreamEvent {
    pub fn function_calls(&self) -> Vec<FunctionCall> {
        let mut out = Vec::new();
        if let Some(v) = &self.response {
            collect_function_calls(v, &mut out);
        }
        if let Some(v) = &self.delta {
            collect_function_calls(v, &mut out);
        }
        if let Some(v) = &self.message {
            collect_function_calls(v, &mut out);
        }
        out
    }
}

fn collect_function_calls(v: &serde_json::Value, out: &mut Vec<FunctionCall>) {
    match v {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::String(t)) = map.get("type") {
                if t == "function" {
                    if let Some(f) = map.get("function") {
                        if let Some(fc) = as_function_call(f) {
                            out.push(fc);
                        }
                    }
                    // Recurse other keys except `function` to find additional, avoid double count
                    for (k, val) in map {
                        if k != "function" {
                            collect_function_calls(val, out);
                        }
                    }
                    return;
                }
            }
            if let Some(fc) = as_function_call(v) {
                out.push(fc);
                return;
            }
            for (_k, val) in map {
                collect_function_calls(val, out);
            }
        }
        serde_json::Value::Array(arr) => {
            for item in arr {
                collect_function_calls(item, out);
            }
        }
        _ => {}
    }
}

fn as_function_call(v: &serde_json::Value) -> Option<FunctionCall> {
    let m = v.as_object()?;
    if let Some(f) = m.get("function") {
        let fobj = f.as_object()?;
        let name = fobj.get("name")?.as_str()?.to_string();
        let args = fobj
            .get("arguments")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        return Some(FunctionCall {
            name,
            arguments: normalize_args(args),
        });
    }
    if let Some(name) = m.get("name").and_then(|s| s.as_str()) {
        let args = m
            .get("arguments")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        return Some(FunctionCall {
            name: name.to_string(),
            arguments: normalize_args(args),
        });
    }
    None
}

fn normalize_args(args: serde_json::Value) -> serde_json::Value {
    match args {
        serde_json::Value::String(s) => {
            serde_json::from_str(&s).unwrap_or(serde_json::Value::String(s))
        }
        other => other,
    }
}
