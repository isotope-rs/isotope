use crate::analyzer::analyzer_trait;
use crate::analyzer::types::AnalysisResults;

use async_trait::async_trait;
use aws_sdk_iam;
use colored::Colorize;
use std::sync::Arc;
pub struct STSAnalyzer {
    pub config: Arc<aws_config::SdkConfig>,
}
#[async_trait]
impl analyzer_trait::Analyzer for STSAnalyzer {
    async fn run(&self) -> Option<Vec<AnalysisResults>> {
        println!(
            "{} {} {}",
            "Running".green(),
            "STS".blue(),
            "analyzer".green()
        );

	    let mut results = vec![AnalysisResults {
		    message: "".to_string(),
            analyzer_name: "".to_string()
	    }];
        let iam = aws_sdk_iam::Client::new(&self.config.clone());
        let list_users_response = iam.list_users().send().await;
        let users = list_users_response.unwrap().users.unwrap_or_default();
        for user in users {
            let username = user.user_name.as_deref().unwrap_or_default();

            // Use IAM to get user's MFA status
            let mfa_devices_response = iam.list_mfa_devices().user_name(username).send().await;
            let mfa_devices = mfa_devices_response.unwrap().mfa_devices.unwrap_or_default();

            if mfa_devices.is_empty() {
                results.push(AnalysisResults{
	               message: format!("MFA is not enabled for user {}", username),
                   analyzer_name: self.get_name()
                });
            }
        }

        Some(results)
    }

    fn get_name(&self) -> String {
        "sts".to_string()
    }
}
