use crate::analyzer::analyzer_trait;
use crate::analyzer::analyzer_trait::Analyzer;
use crate::analyzer::types::AnalysisResults;
use crate::utils;
use async_trait::async_trait;
use aws_sdk_ec2;
use aws_types::sdk_config::SdkConfig;

pub struct EbsAnalyzer {
    pub config: SdkConfig,
}

#[async_trait]
impl analyzer_trait::Analyzer for EbsAnalyzer {
    async fn run(&self) -> Option<Vec<AnalysisResults>> {
        let mut results = Vec::new();
        let ec2 = aws_sdk_ec2::Client::new(&self.config);

        if let Ok(volumes) = ec2.describe_volumes().send().await {
            for volume in volumes.volumes.unwrap_or_default() {
                // Check if the volume is unattached
                if volume.attachments.unwrap().is_empty() {
                    results.push(AnalysisResults {
                        message: format!(
                            "Unattached EBS volume: {}",
                            volume.volume_id.clone().unwrap()
                        ),
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
