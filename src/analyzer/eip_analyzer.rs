use crate::analyzer::analyzer_trait;
use crate::analyzer::analyzer_trait::Analyzer;
use crate::analyzer::types::AnalysisResults;
use async_trait::async_trait;

use aws_types::sdk_config::SdkConfig;

pub struct EipAnalyzer {
    pub config: SdkConfig,
}
#[async_trait]
impl analyzer_trait::Analyzer for EipAnalyzer {
    async fn run(&self) -> Option<Vec<AnalysisResults>> {
        let mut results = Vec::new();
        let ec2 = aws_sdk_ec2::Client::new(&self.config);

        if let Ok(addresses) = ec2.describe_addresses().send().await {
            for address in addresses.addresses.unwrap_or_default() {
                // Check if the Elastic IP is not associated
                if address.association_id.is_none() {
                    results.push(AnalysisResults {
                        message: format!(
                            "Unused Elastic IP: {}",
                            address.public_ip.clone().unwrap()
                        ),
                        advice: "Consider releasing unused Elastic IPs to avoid charges."
                            .to_string(),
                        analyzer_name: self.get_name(),
                    });
                }
            }
        }

        Some(results)
    }

    fn get_name(&self) -> String {
        "eip".to_string()
    }
}
