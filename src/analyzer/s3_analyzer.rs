use std::env;
use aws_types::sdk_config::SdkConfig;
use crate::analyzer::analyzer_trait;
use crate::analyzer::types::AnalysisResults;
use async_trait::async_trait;
use aws_types::region::Region;
use crate::analyzer::analyzer_trait::Analyzer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::utils;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Statement")]
    pub statement: Vec<Statement>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statement {
    #[serde(rename = "Effect")]
    pub effect: String,
    #[serde(rename = "Principal")]
    pub principal: Value,
    #[serde(rename = "Action")]
    pub action: String,
    #[serde(rename = "Resource")]
    pub resource: Value,
    #[serde(rename = "Condition")]
    pub condition: Condition,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    #[serde(rename = "StringEquals")]
    pub string_equals: Option<StringEquals>,
    #[serde(rename = "Bool")]
    pub bool: Option<Bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StringEquals {
    #[serde(rename = "aws:SourceAccount")]
    pub aws_source_account: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bool {
    #[serde(rename = "aws:SecureTransport")]
    pub aws_secure_transport: String,
}

pub struct S3Analyzer {
    pub config: SdkConfig
}
#[async_trait]
impl analyzer_trait::Analyzer for S3Analyzer {
    async fn run(&self) -> Option<Vec<AnalysisResults>> {
        let mut results = vec![AnalysisResults {
            message: "".to_string(),
            analyzer_name: self.get_name(),
            advice: "".to_string(),
        }];
        let s3 = aws_sdk_s3::Client::new(&self.config);
        let s3_response = s3.list_buckets().send().await;

        for bucket in s3_response.unwrap().buckets {
            for b in bucket {
                let bucket_name = b.name.unwrap();
                // Check if the S3 bucket ACL is publicly accessible.
                if let Ok(acl_response)  = s3.get_bucket_acl().bucket(&bucket_name).send().await {
                    for grant in acl_response.grants {
                        if let Some(grantee) = grant.first() {
                            if grantee.clone().grantee.unwrap().uri
                                == Some("http://acs.amazonaws.com/groups/global/AllUsers".to_string())
                            {
                                results.push(AnalysisResults {
                                    message: format!("Publicly accessible S3 bucket {}", &bucket_name),
                                    analyzer_name: self.get_name(),
                                    advice: "".to_string(),
                                });
                            }
                        }
                    }
                }
                let policy_response = s3.get_bucket_policy().bucket(&bucket_name).send().await;
                match policy_response {
                    Ok(response) => {
                        // Check the policy JSON for permissions allowing public access.
                        let policy_json = response.policy.unwrap();
                        // You can parse and analyze the policy JSON if needed.
                        // Deserialize the JSON into a BucketPolicy struct.
                        match serde_json::from_str::<Policy>(&policy_json) {
                            Ok(data) => {
                                for s in data.statement {
                                    if s.principal == "*" {
                                        results.push(AnalysisResults {
                                            message: format!(
                                                "Publicly accessible S3 bucket {}",
                                                &bucket_name
                                            ),
                                            analyzer_name: self.get_name(),
                                            advice: "".to_string(),
                                        });
                                    }
                                }
                            }
                            Err(_e) => (),
                        }
                    }
                    Err(err) => (
                        ),
                }
            }
        }

        Some(results)
    }

    fn get_name(&self) -> String {
        "s3".to_string()
    }
}

#[tokio::test]
async fn get_name_test() {
    let config = utils::load_config().await;

    let s3_analyzer = S3Analyzer {
        config: config
    };
    assert_eq!(s3_analyzer.get_name(), "s3".to_string());
}
