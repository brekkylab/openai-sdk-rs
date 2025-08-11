use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use openai_sdk_rs::{OpenAI, types::{embeddings::{EmbeddingsRequest, EmbeddingInput}, responses::ResponsesRequest}};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

static ATTEMPTS: AtomicUsize = AtomicUsize::new(0);

fn dynamic_response(_: &wiremock::Request) -> ResponseTemplate {
    let n = ATTEMPTS.fetch_add(1, Ordering::SeqCst);
    if n < 2 {
        ResponseTemplate::new(500)
    } else {
        let body = serde_json::json!({
            "object": "list",
            "data": [{"object": "embedding", "index": 0, "embedding": [0.1, 0.2]}],
            "model": "text-embedding-3-small",
            "usage": {"prompt_tokens": 1, "total_tokens": 1}
        });
        ResponseTemplate::new(200).set_body_json(body)
    }
}

#[tokio::test]
async fn retries_until_success() {
    let server = MockServer::start().await;
    ATTEMPTS.store(0, Ordering::SeqCst); // Reset counter

    // Dynamic responder: 2x 500 then 200
    Mock::given(method("POST")).and(path("/v1/embeddings"))
        .respond_with(dynamic_response)
        .mount(&server)
        .await;


    let base = format!("{}/v1/", server.uri());
    let client = OpenAI::builder()
        .api_key("api_key".into())
        .base_url(base)
        .max_retries(5)
        .retry_base_delay(Duration::from_millis(1))
        .build()
        .unwrap();

    let req = EmbeddingsRequest { model: "text-embedding-3-small".into(), input: EmbeddingInput::from("hi"), user: None };
    let resp = client.embeddings(req).await.unwrap();
    assert_eq!(resp.data.len(), 1);
    assert!(ATTEMPTS.load(Ordering::SeqCst) >= 3);
}

#[tokio::test]
async fn sse_streaming_responses() {

    let server = MockServer::start().await;

    // 修复SSE响应格式，添加必需的type字段
    let body = "data: {\"type\":\"content\",\"output_text\":\"hello \"}\n\n\
data: {\"type\":\"content\",\"output_text\":\"world\"}\n\n\
data: [DONE]\n";

    Mock::given(method("POST")).and(path("/v1/responses"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_string(body)
            .insert_header("content-type", "text/event-stream"))
        .mount(&server)
        .await;

    let base = format!("{}/v1/", server.uri());
    let client = OpenAI::builder()
        .api_key("api_key".into())
        .base_url(base)
        .build()
        .unwrap();
   let r = ResponsesRequest::text("gpt-4o-mini", "irrelevant");
   match client.responses_stream_text(r).await{
        Ok(text) => {
            println!("{}", text);
            assert_eq!(text, "hello world");
        },
        Err(e) => panic!("Failed to stream responses: {}", e),
    }
}
