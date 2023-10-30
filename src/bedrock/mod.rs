use aws_config::SdkConfig;
use aws_sdk_bedrock;
pub struct BedrockClient {
	client: aws_sdk_bedrock::Client
}

impl BedrockClient {
	pub fn new(config: SdkConfig) -> Self {
		Self{
			client: aws_sdk_bedrock::Client::new(&config)
		}
	}
}