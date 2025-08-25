use std::time::Duration;

use async_stream::try_stream;
use futures_util::{StreamExt, TryStreamExt};
use reqwest::{header, Client as HttpClient, StatusCode, Url};
use serde::de::DeserializeOwned;

use crate::error::{ApiError, ApiErrorEnvelope, Error};
use crate::types::chat::{ChatCompletionChunk, ChatCompletionRequest, ChatCompletionResponse};
use crate::types::embeddings::{EmbeddingsRequest, EmbeddingsResponse};
use crate::types::files::{FileDeleteResponse, FileListResponse, FileObject};
use crate::types::images::{ImageGenerationRequest, ImageGenerationResponse};
use crate::types::responses::{ResponseStreamEvent, ResponsesRequest, ResponsesResponse};
use crate::utils::sleep;
use crate::utils::BoxStream;

const DEFAULT_BASE_URL: &str = "https://api.openai.com";

#[derive(Clone)]
pub struct OpenAI {
    http: HttpClient,
    base_url: Url,
    api_key: String,
    org: Option<String>,
    project: Option<String>,
    max_retries: u32,
    retry_base_delay_ms: u64,
}

impl std::fmt::Debug for OpenAI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAI")
            .field("base_url", &self.base_url)
            .field("org", &self.org)
            .field("project", &self.project)
            .finish_non_exhaustive()
    }
}

impl OpenAI {
    pub fn base_url(&self) -> String {
        self.base_url.as_str().to_string()
    }

    pub fn new<S: Into<String>>(api_key: S) -> Result<Self, Error> {
        Self::builder().api_key(api_key.into()).build()
    }

    pub fn with_http_client<S: Into<String>>(http: HttpClient, api_key: S) -> Result<Self, Error> {
        Self::builder()
            .http_client(http)
            .api_key(api_key.into())
            .build()
    }

    pub fn from_env() -> Result<Self, Error> {
        let api_key = std::env::var("OPENAI_API_KEY").map_err(|_| Error::MissingApiKey)?;
        let mut b = Self::builder().api_key(api_key);
        if let Ok(o) = std::env::var("OPENAI_ORG_ID") {
            b = b.org(o);
        }
        if let Ok(p) = std::env::var("OPENAI_PROJECT_ID") {
            b = b.project(p);
        }
        if let Ok(u) = std::env::var("OPENAI_BASE_URL") {
            b = b.base_url(u);
        }
        b.build()
    }

    pub fn builder() -> OpenAIBuilder {
        OpenAIBuilder::default()
    }

    pub async fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, Error> {
        self.post_json("/v1/chat/completions", &req).await
    }

    pub async fn embeddings(&self, req: EmbeddingsRequest) -> Result<EmbeddingsResponse, Error> {
        self.post_json("/v1/embeddings", &req).await
    }

    pub async fn chat_completion_stream(
        &self,
        mut req: ChatCompletionRequest,
    ) -> Result<BoxStream<'static, Result<ChatCompletionChunk, Error>>, Error> {
        req.stream = Some(true);
        self.post_sse("/v1/chat/completions", &req).await
    }

    pub async fn responses(&self, req: ResponsesRequest) -> Result<ResponsesResponse, Error> {
        self.post_json("/v1/responses", &req).await
    }

    pub async fn responses_stream(
        &self,
        mut req: ResponsesRequest,
    ) -> Result<BoxStream<'static, Result<ResponseStreamEvent, Error>>, Error> {
        req.stream = Some(true);
        self.post_sse("/v1/responses", &req).await
    }

    pub async fn images_generate(
        &self,
        req: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse, Error> {
        self.post_json("/v1/images/generations", &req).await
    }

    pub async fn files_list(&self) -> Result<FileListResponse, Error> {
        self.get_json("/v1/files").await
    }

    pub async fn files_upload_bytes(
        &self,
        filename: &str,
        bytes: Vec<u8>,
        purpose: &str,
    ) -> Result<FileObject, Error> {
        let url = self.base_url.join("/v1/files").expect("valid path");
        let form = reqwest::multipart::Form::new()
            .text("purpose", purpose.to_string())
            .part(
                "file",
                reqwest::multipart::Part::bytes(bytes).file_name(filename.to_string()),
            );

        let mut req = self
            .http
            .post(url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .multipart(form);
        if let Some(org) = &self.org {
            req = req.header("OpenAI-Organization", org);
        }
        if let Some(project) = &self.project {
            req = req.header("OpenAI-Project", project);
        }
        let resp = self
            .execute_with_retry(|| req.try_clone().expect("req clone"), false)
            .await?;
        let status = resp.status();
        if status.is_success() {
            Ok(resp.json::<FileObject>().await?)
        } else {
            Self::map_api_error(status, resp).await
        }
    }

    pub async fn files_download(&self, file_id: &str) -> Result<Vec<u8>, Error> {
        let mk = || {
            let url = self
                .base_url
                .join(&format!("/v1/files/{}/content", file_id))
                .expect("valid path");
            let mut req = self
                .http
                .get(url)
                .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key));
            if let Some(org) = &self.org {
                req = req.header("OpenAI-Organization", org);
            }
            if let Some(project) = &self.project {
                req = req.header("OpenAI-Project", project);
            }
            req
        };
        let resp = self.execute_with_retry(mk, false).await?;
        let status = resp.status();
        if status.is_success() {
            Ok(resp.bytes().await?.to_vec())
        } else {
            Self::map_api_error(status, resp).await
        }
    }

    pub async fn files_delete(&self, file_id: &str) -> Result<FileDeleteResponse, Error> {
        let mk = || {
            let url = self
                .base_url
                .join(&format!("/v1/files/{}", file_id))
                .expect("valid path");
            let mut req = self
                .http
                .delete(url)
                .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key));
            if let Some(org) = &self.org {
                req = req.header("OpenAI-Organization", org);
            }
            if let Some(project) = &self.project {
                req = req.header("OpenAI-Project", project);
            }
            req
        };
        let resp = self.execute_with_retry(mk, false).await?;
        let status = resp.status();
        if status.is_success() {
            Ok(resp.json::<FileDeleteResponse>().await?)
        } else {
            Self::map_api_error(status, resp).await
        }
    }

    pub async fn chat_completion_stream_text(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<String, Error> {
        let mut stream = self.chat_completion_stream(req).await?;
        let mut out = String::new();
        while let Some(chunk) = stream.try_next().await? {
            if let Some(text) = chunk
                .choices
                .first()
                .and_then(|c| c.delta.content.as_deref())
            {
                out.push_str(text);
            }
        }
        Ok(out)
    }

    pub async fn responses_stream_text(&self, req: ResponsesRequest) -> Result<String, Error> {
        let mut stream = self.responses_stream(req).await?;

        let mut out = String::new();
        while let Some(ev) = stream.next().await {
            let ev = ev?;
            if let Some(text) = ev.clone().output_text.as_deref() {
                out.push_str(text);
            } else if let Some(d) = ev
                .delta
                .as_ref()
                .and_then(|v| v.get("output_text"))
                .and_then(|v| v.as_str())
            {
                out.push_str(d);
            }
        }

        Ok(out)
    }

    async fn post_json<TReq: serde::Serialize, TResp: DeserializeOwned>(
        &self,
        path: &str,
        body: &TReq,
    ) -> Result<TResp, Error> {
        let mk = || {
            let url = self.base_url.join(path).expect("valid path");
            let mut req = self
                .http
                .post(url)
                .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
                .json(body);
            if let Some(org) = &self.org {
                req = req.header("OpenAI-Organization", org);
            }
            if let Some(project) = &self.project {
                req = req.header("OpenAI-Project", project);
            }
            req
        };

        let resp = self.execute_with_retry(mk, false).await?;
        let status = resp.status();
        if status.is_success() {
            Ok(resp.json::<TResp>().await?)
        } else {
            Self::map_api_error(status, resp).await
        }
    }

    async fn get_json<TResp: DeserializeOwned>(&self, path: &str) -> Result<TResp, Error> {
        let mk = || {
            let url = self.base_url.join(path).expect("valid path");
            let mut req = self
                .http
                .get(url)
                .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key));
            if let Some(org) = &self.org {
                req = req.header("OpenAI-Organization", org);
            }
            if let Some(project) = &self.project {
                req = req.header("OpenAI-Project", project);
            }
            req
        };
        let resp = self.execute_with_retry(mk, false).await?;
        let status = resp.status();
        if status.is_success() {
            Ok(resp.json::<TResp>().await?)
        } else {
            Self::map_api_error(status, resp).await
        }
    }

    async fn post_sse<TReq: serde::Serialize, TEvent: DeserializeOwned + Send + 'static>(
        &self,
        path: &str,
        body: &TReq,
    ) -> Result<BoxStream<'static, Result<TEvent, Error>>, Error> {
        let mk = || {
            let url = self.base_url.join(path).expect("valid path");
            let mut req = self
                .http
                .post(url)
                .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
                .header(header::ACCEPT, "text/event-stream")
                .json(body);
            if let Some(org) = &self.org {
                req = req.header("OpenAI-Organization", org);
            }
            if let Some(project) = &self.project {
                req = req.header("OpenAI-Project", project);
            }
            req
        };

        let resp = self.execute_with_retry(mk, true).await?;
        let status = resp.status();
        if !status.is_success() {
            return Self::map_api_error(status, resp).await;
        }
        let res = Self::sse_json_stream::<TEvent>(resp);
        Ok(res)
    }

    fn sse_json_stream<T: DeserializeOwned + Send + 'static>(
        resp: reqwest::Response,
    ) -> BoxStream<'static, Result<T, Error>> {
        let stream = try_stream! {
            let mut buf: Vec<u8> = Vec::new();
            let mut byte_stream = resp.bytes_stream();
            while let Some(chunk) = futures_util::StreamExt::next(&mut byte_stream).await {
                let chunk = chunk?;
                buf.extend_from_slice(&chunk);

                let mut start = 0usize;
                for i in 0..buf.len() {
                    if buf[i] == b'\n' {
                        let mut line = &buf[start..i];
                        start = i + 1;
                        if !line.is_empty() && line[line.len()-1] == b'\r' {
                            line = &line[..line.len()-1];
                        }
                        if line.is_empty() { continue; }
                        if line[0] == b':' { continue; }
                        if let Some(rest) = line.strip_prefix(b"data: ") {
                            if rest == b"[DONE]" { return; }
                            let text = String::from_utf8(rest.to_vec()).unwrap_or_default();
                            let val: T = serde_json::from_str(&text)?;
                            yield val;
                        }
                    }
                }
                if start > 0 { buf.drain(0..start); }
            }
        };
        Box::pin(stream)
    }

    #[cfg(test)]
    fn sse_extract_data_lines(text: &str) -> Vec<String> {
        text.lines()
            .filter_map(|l| {
                let l = l.trim_end_matches('\r');
                if l.is_empty() || l.starts_with(':') {
                    return None;
                }
                if let Some(rest) = l.strip_prefix("data: ") {
                    if rest == "[DONE]" {
                        return None;
                    }
                    return Some(rest.to_string());
                }
                None
            })
            .collect()
    }

    async fn map_api_error<TResp>(
        status: StatusCode,
        resp: reqwest::Response,
    ) -> Result<TResp, Error> {
        let text = resp.text().await.unwrap_or_default();
        if let Ok(env) = serde_json::from_str::<ApiErrorEnvelope>(&text) {
            let mut api: ApiError = env.into();
            api.status = Some(status.as_u16());
            Err(Error::Api(api))
        } else {
            Err(Error::UnexpectedStatus {
                status: status.as_u16(),
                body: text,
            })
        }
    }
}

impl OpenAI {
    async fn execute_with_retry<F>(&self, mk: F, _sse: bool) -> Result<reqwest::Response, Error>
    where
        F: Fn() -> reqwest::RequestBuilder,
    {
        let mut attempt = 0u32;
        loop {
            let req = mk();
            let res = req.send().await;
            return match res {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() {
                        return Ok(resp);
                    }
                    if self.should_retry_status(status) && attempt < self.max_retries {
                        let delay =
                            self.retry_delay(attempt, resp.headers().get(header::RETRY_AFTER));
                        attempt += 1;
                        sleep(delay).await;
                        continue;
                    }
                    Ok(resp)
                }
                Err(e) => {
                    println!("Request error: {}", e);
                    if self.is_retryable_error(&e) && attempt < self.max_retries {
                        let delay = self.retry_delay(attempt, None);
                        attempt += 1;
                        sleep(delay).await;
                        continue;
                    }
                    Err(Error::Http(e))
                }
            };
        }
    }

    fn should_retry_status(&self, status: StatusCode) -> bool {
        status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error()
    }

    fn retry_delay(
        &self,
        attempt: u32,
        retry_after: Option<&header::HeaderValue>,
    ) -> std::time::Duration {
        if let Some(v) = retry_after {
            if let Ok(s) = v.to_str() {
                if let Ok(secs) = s.parse::<u64>() {
                    return Duration::from_secs(secs);
                }
            }
        }
        let base = self.retry_base_delay_ms;
        let backoff = base.saturating_mul(1u64 << attempt.min(8));
        Duration::from_millis(backoff)
    }

    fn is_retryable_error(&self, e: &reqwest::Error) -> bool {
        let mut result = e.is_timeout() || e.is_request();
        #[cfg(not(target_arch = "wasm32"))]
        {
            result = result || e.is_connect()
        }
        result = result;
        result
    }
}

#[derive(Default)]
pub struct OpenAIBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    org: Option<String>,
    project: Option<String>,
    timeout: Option<Duration>,
    user_agent: Option<String>,
    max_retries: Option<u32>,
    retry_base_delay_ms: Option<u64>,
    http: Option<HttpClient>,
    proxy: Option<String>,
}

impl OpenAIBuilder {
    pub fn api_key(mut self, key: String) -> Self {
        self.api_key = Some(key);
        self
    }
    pub fn base_url<S: Into<String>>(mut self, url: S) -> Self {
        self.base_url = Some(url.into());
        self
    }
    pub fn org<S: Into<String>>(mut self, org: S) -> Self {
        self.org = Some(org.into());
        self
    }
    pub fn project<S: Into<String>>(mut self, project: S) -> Self {
        self.project = Some(project.into());
        self
    }
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    pub fn user_agent<S: Into<String>>(mut self, ua: S) -> Self {
        self.user_agent = Some(ua.into());
        self
    }
    pub fn max_retries(mut self, n: u32) -> Self {
        self.max_retries = Some(n);
        self
    }
    pub fn retry_base_delay(mut self, dur: Duration) -> Self {
        self.retry_base_delay_ms = Some(dur.as_millis() as u64);
        self
    }
    pub fn http_client(mut self, client: HttpClient) -> Self {
        self.http = Some(client);
        self
    }
    pub fn proxy<S: Into<String>>(mut self, url: S) -> Self {
        self.proxy = Some(url.into());
        self
    }

    pub fn build(self) -> Result<OpenAI, Error> {
        let api_key = self.api_key.ok_or(Error::MissingApiKey)?;
        let base_url_str = self
            .base_url
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
        let base_url = Url::parse(&base_url_str)?;

        let http = if let Some(custom) = self.http {
            custom
        } else {
            let mut headers = header::HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/json"),
            );

            let mut http = HttpClient::builder().default_headers(headers);

            if let Some(ua) = self.user_agent {
                http = http.user_agent(ua);
            } else {
                http = http.user_agent(format!(
                    "openai-sdk-rs/{} (+https://crates.io/crates/openai-sdk)",
                    env!("CARGO_PKG_VERSION")
                ));
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                http = http.gzip(true).brotli(true);

                if let Some(t) = self.timeout {
                    http = http.timeout(t);
                }

            if let Some(px) = self.proxy {
                if let Ok(proxy) = reqwest::Proxy::all(px) {
                    http = http.proxy(proxy);
                }
            }
            }

            http.build()?
        };

        Ok(OpenAI {
            http,
            base_url,
            api_key,
            org: self.org,
            project: self.project,
            max_retries: self.max_retries.unwrap_or(3),
            retry_base_delay_ms: self.retry_base_delay_ms.unwrap_or(200),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::OpenAI;

    #[test]
    fn sse_extracts_data_lines() {
        let input =
            "event: message\n:data line as comment\ndata: {\"a\":1}\n\nretry: 5000\ndata: [DONE]\n";
        let lines = OpenAI::sse_extract_data_lines(input);
        assert_eq!(lines, vec!["{\"a\":1}".to_string()]);
    }
}
