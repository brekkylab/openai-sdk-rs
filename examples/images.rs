use openai_sdk_rs::{
    types::images::{ImageGenerationRequest, ImageResponseFormat},
    OpenAI,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenAI::from_env()?;
    let req = ImageGenerationRequest {
        model: "dall-e-3".into(),
        prompt: "A cute robot reading Rust code".into(),
        n: Some(1),
        size: Some("1024x1024".into()),
        response_format: Some(ImageResponseFormat::Url),
    };
    let resp = client.images_generate(req).await?;
    if let Some(img) = resp.data.get(0) {
        if let Some(b64) = &img.b64_json {
            println!("b64 length: {}", b64.len());
        }
        if let Some(url) = &img.url {
            println!("url: {}", url);
        }
    }
    Ok(())
}
