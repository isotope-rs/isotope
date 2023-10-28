use crate::analyzer::analyzer_trait;
use crate::analyzer::types::AnalysisResults;
use async_trait::async_trait;
use aws_sdk_s3;
use colored::Colorize;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::{Value};

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
    pub config: Arc<aws_config::SdkConfig>,
}
#[async_trait]
impl analyzer_trait::Analyzer for S3Analyzer {

    async fn run(&self) -> Option<Vec<AnalysisResults>> {
        println!(
            "{} {} {}",
            "Running".green(),
            "S3".blue(),
            "analyzer".green()
        );
        let mut results = vec![AnalysisResults {
            message: "".to_string(),
            analyzer_name: self.get_name()
        }];

        let s3 = aws_sdk_s3::Client::new(&self.config);

        let s3_response = s3.list_buckets().send().await;

        for bucket in s3_response.unwrap().buckets {
            for b in bucket {
                let bucket_name = b.name.unwrap();
                // Check if the S3 bucket ACL is publicly accessible.
                let acl_response = s3.get_bucket_acl().bucket(&bucket_name).send().await;
                for grant in acl_response.unwrap().grants {
                    if let Some(grantee) = grant.first() {
                        if grantee.clone().grantee.unwrap().uri
                            == Some("http://acs.amazonaws.com/groups/global/AllUsers".to_string())
                        {
                            results.push(AnalysisResults{
                                message: format!("Publicly accessible S3 bucket {}", &bucket_name),
                                analyzer_name: self.get_name()
                            });
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
                        match  serde_json::from_str::<Policy>(&policy_json) {
                            Ok(data) => {

                                for s in data.statement {
                                   if s.principal == "*" {
                                       results.push(AnalysisResults{
                                           message: format!("Publicly accessible S3 bucket {}", &bucket_name),
                                           analyzer_name: self.get_name()
                                       });
                                   }
                                }
                            },
                            Err(_e) => ()
                        }
                    }
                    Err(_err) => ()
                }
            }
        }

        Some(results)
    }

    fn get_name(&self) -> String {
        "S3".to_string()
    }
}
