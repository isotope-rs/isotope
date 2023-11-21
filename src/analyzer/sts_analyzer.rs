use crate::analyzer::analyzer_trait;
use crate::analyzer::analyzer_trait::Analyzer;
use crate::analyzer::types::AnalysisResults;
use async_trait::async_trait;
use aws_types::sdk_config::SdkConfig;



use crate::utils;

pub struct STSAnalyzer {
    pub config: SdkConfig,
}
#[async_trait]
impl analyzer_trait::Analyzer for STSAnalyzer {
    async fn run(&self) -> Option<Vec<AnalysisResults>> {
        let mut results = vec![AnalysisResults {
            message: "".to_string(),
            analyzer_name: "".to_string(),
            advice: "".to_string(),
        }];
        let _config = utils::load_config().await;
        let iam = aws_sdk_iam::Client::new(&self.config);
        let list_users_response = iam.list_users().send().await;
        let users = list_users_response.unwrap().users;
        for user in users {
            let username = user.user_name;

            // Use IAM to get user's MFA status
            let mfa_devices_response = iam.list_mfa_devices().user_name(&username).send().await;
            let mfa_devices = mfa_devices_response.unwrap().mfa_devices;

            if mfa_devices.is_empty() {
                results.push(AnalysisResults {
                    message: format!("MFA is not enabled for user {}", &username),
                    analyzer_name: self.get_name(),
                    advice: "".to_string(),
                });
            }
        }

        Some(results)
    }

    fn get_name(&self) -> String {
        "sts".to_string()
    }
}
