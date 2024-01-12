use crate::analyzer::analyzer_trait;
use crate::analyzer::types::AnalysisResults;
use async_trait::async_trait;

use aws_types::sdk_config::SdkConfig;
use colored::Colorize;

pub struct SecurityGroupsAnalyzer {
    pub config: SdkConfig,
}

#[async_trait]
impl analyzer_trait::Analyzer for SecurityGroupsAnalyzer {
    async fn run(&self) -> Option<Vec<AnalysisResults>> {
        let mut results = Vec::new();
        let client = aws_sdk_ec2::Client::new(&self.config);
        let response = client.describe_security_groups().send().await;
        match response {
            Ok(x) => {
                for group in x.security_groups.unwrap_or_default() {
                    if has_wide_open_rules(&group.ip_permissions) {
                        results.push(AnalysisResults {
                            message: format!(
                                "Insecure security group: {}",
                                group.group_id.clone().unwrap()
                            ),
                            advice: "".to_string(),
                            analyzer_name: self.get_name(),
                        });
                    }
                }
            }
            Err(e) => {
                println!("{}", e.to_string().as_str())
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
            if rule.from_port.is_none() && rule.clone().to_port.is_none() {
                // Check if the ip_range CIDR blocks are wide open
                if let Some(ip_ranges) = &rule.ip_ranges {
                    for ip_range in ip_ranges {
                        if ip_range.cidr_ip == Some("0.0.0.0/0".to_string()) {
                            return true;
                        }
                    }
                }
                if let Some(ipv6_ranges) = &rule.ipv6_ranges {
                    for ipv6_range in ipv6_ranges {
                        if ipv6_range.cidr_ipv6 == Some("::/0".to_string()) {
                            return true;
                        }
                    }
                }
                return true; // Wide-open rule found
            }
        }
    }

    false
}
