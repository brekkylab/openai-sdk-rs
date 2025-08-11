use openai_sdk_rs::types::responses::{ResponseStreamEvent, ResponsesResponse};

#[test]
fn extract_function_calls_from_response() {
    let v = serde_json::json!({
        "id": "res_1", "object": "responses.response", "created": 0u64, "model": "gpt-4o-mini",
        "output": [
            {"type": "function", "function": {"name": "calc", "arguments": "{\"a\":1,\"b\":2}"}},
            {"name": "echo", "arguments": {"text": "hi"}}
        ]
    });
    let resp: ResponsesResponse = serde_json::from_value(v).unwrap();
    let calls = resp.function_calls();
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].name, "calc");
    assert_eq!(calls[0].arguments["a"], 1);
    assert_eq!(calls[1].name, "echo");
    assert_eq!(calls[1].arguments["text"], "hi");
}

#[test]
fn extract_function_calls_from_stream_event() {
    let v = serde_json::json!({
        "type": "response.output_delta",
        "delta": {"type": "function", "function": {"name": "calc", "arguments": "{\"x\":10}"}}
    });
    let ev: ResponseStreamEvent = serde_json::from_value(v).unwrap();
    let calls = ev.function_calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "calc");
    assert_eq!(calls[0].arguments["x"], 10);
}

#[test]
fn output_json_parses_text() {
    let v = serde_json::json!({
        "id": "res_1", "object": "responses.response", "created": 0u64, "model": "gpt",
        "output": {"text": "{\"ok\":true}"}
    });
    let resp: ResponsesResponse = serde_json::from_value(v).unwrap();
    let j = resp.output_json().unwrap();
    assert_eq!(j["ok"], true);
}
