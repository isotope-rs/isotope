use aws_sdk_ec2;
use async_trait::async_trait;
use aws_sdk_rds;
use crate::analyzer::analyzer_trait;
use crate::analyzer::types::AnalysisResults;
use colored::Colorize;
use crate::analyzer::analyzer_trait::Analyzer;

pub struct EbsAnalyzer {
	pub config: aws_config::SdkConfig,
}

#[async_trait]
impl analyzer_trait::Analyzer for EbsAnalyzer {
	async fn run(&self) -> Option<Vec<AnalysisResults>> {
		println!(
			"{} {} {}",
			"Running".green(),
			"EBS Volume".blue(),
			"analyzer".green()
		);
		let mut results = Vec::new();
		// TODO: Weird idiosyncrasy of the EC2 client
		 let mut config = aws_types::sdk_config::SdkConfig::builder().build();

		let ec2 = aws_sdk_ec2::Client::new(&config);

		if let Ok(volumes) = ec2.describe_volumes().send().await {
			for volume in volumes.volumes.unwrap_or_default() {
				// Check if the volume is unattached
				if volume.attachments.unwrap().is_empty() {
					results.push(AnalysisResults {
						message: format!("Unattached EBS volume: {}", volume.volume_id.clone().unwrap()),
						advice: "".to_string(),
						analyzer_name: self.get_name(),
					});
				}
			}
		}

		Some(results)
	}

	fn get_name(&self) -> String {
		"ebs".to_string()
	}
}

#[tokio::test]
async fn get_name_test() {
	let ebs_analyzer = EbsAnalyzer {
		config : aws_config::SdkConfig::builder().build(),
	};
	assert_eq!(ebs_analyzer.get_name(), "ebs".to_string());
}