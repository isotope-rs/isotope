use aws_sdk_bedrockruntime::primitives::Blob;

use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

use aws_types::region::Region;

mod prompt;

#[derive(Serialize)]
struct ClaudParams {
    prompt: String,
    max_tokens_to_sample: usize,
}

#[derive(Deserialize)]
struct ClaudeOutput {
    completion: String,
}
impl ClaudParams {
    fn new(question: &str) -> Self {
        Self {
            prompt: format!("\n\nHuman:{}\n\nAssistant:", question),
            max_tokens_to_sample: 1000,
        }
    }
}

impl From<ClaudParams> for Blob {
    fn from(val: ClaudParams) -> Self {
        Blob::new(serde_json::to_string(&val).unwrap())
    }
}
pub struct BedrockClient {
    region: Region,
    bedrock_model: String,
}

impl BedrockClient {
    pub fn new() -> Self {
        Self {
            region: Region::new(env::var("BEDROCK_REGION").unwrap().clone()),
            bedrock_model: env::var("BEDROCK_MODEL").unwrap().clone(),
        }
    }
    pub async fn enrich(&self, prompt: String) -> Result<String, Box<dyn Error>> {
        // force the config rejoin be set
        let config = aws_config::from_env()
            .region(self.region.clone())
            .load()
            .await;
        let client = aws_sdk_bedrockruntime::Client::new(&config);
        let question = format!("{} {}", prompt::DEFAULT_PROMPT, prompt.as_str());

        let response = client
            .invoke_model()
            .model_id(self.bedrock_model.clone())
            .content_type("application/json")
            .body(ClaudParams::new(question.as_str()).into())
            .send()
            .await?
            .body;

        let mut response_capture = ClaudeOutput {
            completion: "".to_string(),
        };
        let data = response;
        let data = serde_json::from_slice::<ClaudeOutput>(data.as_ref()).expect("invalid schema");
        response_capture.completion = data.completion.clone();
        Ok(response_capture.completion)
    }
}
