use async_trait::async_trait;
use aws_sdk_iam::Client;
use aws_types::sdk_config::SdkConfig;
use crate::analyzer::analyzer_trait::Analyzer;
use crate::analyzer::types::AnalysisResults;
use chrono::{TimeZone, Utc};
use chrono::Duration;
pub struct IamAnalyzer {
	pub config: SdkConfig
}

#[async_trait]
impl Analyzer for IamAnalyzer {

	async fn run(&self) -> Option<Vec<AnalysisResults>> {
		let mut results = Vec::new();
		let iam = Client::new(&self.config);

		// Check for unused access keys
		if let Ok(keys) = iam.list_access_keys().send().await {
			for key in keys.access_key_metadata {
				if let Some(create_date) = key.create_date {
					if !key.access_key_id.as_ref().unwrap().is_empty(){
						if let Some(aws_create_date) = key.create_date {
							let create_date = Utc.timestamp(aws_create_date.secs(), 0);
							if create_date < Utc::now() - Duration::days(90) {
								results.push(AnalysisResults {
									message: format!("Unused access key: {}", key.access_key_id.as_ref().unwrap()),
									advice: "Consider deleting unused access keys".to_string(),
									analyzer_name: self.get_name(),
								});
							}
						}
					}
				}
			}
		}

		Some(results)
	}

	fn get_name(&self) -> String {
		"iam".to_string()
	}
}