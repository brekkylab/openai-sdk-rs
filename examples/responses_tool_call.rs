use openai_sdk::{
    types::responses::{ResponsesRequest, ToolSpec},
    OpenAI,
};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenAI::from_env()?;

    // 定义一个简单的天气查询工具
    let weather_tool = ToolSpec {
        type_: "function".to_string(),
        name: "get_weather".to_string(),
        description: Some("Get the current weather for a given location".to_string()),
        parameters: Some(json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city and state, e.g. San Francisco, CA"
                },
                "unit": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"],
                    "description": "The temperature unit to use"
                }
            },
            "required": ["location"]
        })),
    };

    // 定义一个计算器工具
    let calculator_tool = ToolSpec {
        type_: "function".to_string(),
        name: "calculate".to_string(),
        description: Some("Perform basic mathematical calculations".to_string()),
        parameters: Some(json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "The mathematical expression to evaluate, e.g. '2 + 3 * 4'"
                }
            },
            "required": ["expression"]
        })),
    };

    // 创建请求，要求使用工具
    let mut req = ResponsesRequest::text(
        "gpt-4o-mini",
        "What's the weather like in Tokyo? Also, what's 25 * 17?",
    );

    // 添加工具
    req.tools = Some(vec![weather_tool, calculator_tool]);
    req.tool_choice = Some(json!("auto")); // 让模型自动选择是否使用工具

    println!("Making request with tool calling...");
    let resp = client.responses(req).await?;

    println!("\n=== Response ===");
    println!("ID: {}", resp.id);
    println!("Model: {}", resp.model);

    // 检查是否有函数调用
    let function_calls = resp.function_calls();
    if !function_calls.is_empty() {
        println!("\n=== Function Calls ===");
        for (i, call) in function_calls.iter().enumerate() {
            println!("Call #{}: {}", i + 1, call.name);
            println!(
                "Arguments: {}",
                serde_json::to_string_pretty(&call.arguments)?
            );

            // 模拟执行函数调用
            match call.name.as_str() {
                "get_weather" => {
                    if let Some(location) = call.arguments.get("location").and_then(|v| v.as_str())
                    {
                        let unit = call
                            .arguments
                            .get("unit")
                            .and_then(|v| v.as_str())
                            .unwrap_or("celsius");
                        let temp = if unit == "fahrenheit" {
                            "72°F"
                        } else {
                            "22°C"
                        };
                        println!("→ Mock weather result for {}: Sunny, {}", location, temp);
                    }
                }
                "calculate" => {
                    if let Some(expr) = call.arguments.get("expression").and_then(|v| v.as_str()) {
                        // 简单的计算（仅用于演示）
                        let result = match expr {
                            "25 * 17" => 425,
                            "15 * 23" => 345,
                            "2 + 3 * 4" => 14,
                            _ => 42, // 默认值
                        };
                        println!("→ Calculation result: {} = {}", expr, result);
                    }
                }
                _ => {
                    println!("→ Unknown function: {}", call.name);
                }
            }
        }
    } else {
        println!("\nNo function calls in response.");
    }

    // 显示文本输出
    if let Some(text) = resp.output_text() {
        println!("\n=== Text Output ===");
        println!("{}", text);
    } else {
        println!("\n=== Raw Output (JSON) ===");
        if let Some(output) = &resp.output {
            println!("{}", serde_json::to_string_pretty(output)?);
        }
    }

    Ok(())
}
