use async_trait::async_trait;
use crate::analyzer::analyzer_trait;
use crate::analyzer::types::AnalysisResults;
use colored::Colorize;
use aws_sdk_ec2;
pub struct SecurityGroupsAnalyzer {
}

#[async_trait]
impl analyzer_trait::Analyzer for SecurityGroupsAnalyzer {
	async fn run(&self) -> Option<Vec<AnalysisResults>> {


		return None;

		println!(
			"{} {} {}",
			"Running".green(),
			"Security Groups".blue(),
			"analyzer".green()
		);
		let mut results = Vec::new();
		let config = aws_types::sdk_config::SdkConfig::builder().build();
		let client = aws_sdk_ec2::Client::new(&config);
		let response =  client.describe_security_groups().send().await;
		match response {
			Ok(x) => {
				for group in x.security_groups.unwrap_or_default() {
					if has_wide_open_rules(&group.ip_permissions) {
						results.push(AnalysisResults {
							message: format!("Insecure security group: {}", group.group_id.clone().unwrap()),
							advice: "".to_string(),
							analyzer_name: self.get_name(),
						});
					}
				}
			},
			Err(e) => {
				println!("{}",e.to_string().as_str())
			}
		}

		Some(results)
	}

	fn get_name(&self) -> String {
		"sg".to_string()
	}
}

fn has_wide_open_rules(permissions: &Option<Vec<aws_sdk_ec2::types::IpPermission>>) -> bool {
	if let Some(rules) = permissions {
		for rule in rules {
			if rule.from_port.is_none()
				&& rule.clone().to_port.is_none()
				&& rule.clone().ip_ranges.unwrap().is_empty()
				&& rule.clone().ipv6_ranges.unwrap().is_empty()
				&& rule.clone().user_id_group_pairs.unwrap().is_empty()
			{
				return true; // Wide-open rule found
			}
		}
	}

	false
}