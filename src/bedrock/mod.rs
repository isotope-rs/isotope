use std::env;
use std::error::Error;
use aws_config::{ConfigLoader, SdkConfig};
use aws_sdk_bedrockruntime::primitives::Blob;
use aws_config::meta::region::{ProvideRegion, RegionProviderChain};
use serde::{Deserialize, Serialize};
use std::io::{stdout, Write};
use aws_sdk_config::config::Region;

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

impl Into<Blob> for ClaudParams {
	fn into(self) -> Blob {
		Blob::new(serde_json::to_string(&self).unwrap())
	}
}
pub struct BedrockClient {
	config: aws_config::SdkConfig
}

impl BedrockClient {
	pub fn new(config: SdkConfig) -> Self {

		Self{
			config: config
		}
	}
	pub async fn enrich(&self, prompt: String) -> Result<String,Box<dyn Error>> {

		// force the config rejoin be set
		let bedrock_region = env::var("BEDROCK_REGION")?.clone();
		let conf = aws_config::from_env()
			.region(Region::new(bedrock_region))
			.load()
			.await;
		let client =  aws_sdk_bedrockruntime::Client::new(&conf);
		let bedrock_model = env::var("BEDROCK_MODEL")?;

		let question = format!("{} {}",prompt::DEFAULT_PROMPT, prompt.as_str());

		let mut response = client
			.invoke_model()
			.model_id(bedrock_model)
			.content_type("application/json")
			.body(ClaudParams::new(question.as_str()).into())
			.send()
			.await?
			.body;



		let mut response_capture = ClaudeOutput{
			completion: "".to_string()
		};

		let data = response.unwrap();
		let data = serde_json::from_slice::<ClaudeOutput>(data.as_ref()).expect("invalid schema");
		response_capture.completion = data.completion.clone();
		Ok(response_capture.completion)
	}
}