use crate::analyzer::analyzer_trait;
use crate::analyzer::types::AnalysisResults;
use crate::utils;
use async_trait::async_trait;
use colored::Colorize;
use std::sync::Arc;
use aws_sdk_s3;
const role_name: &str = "DetectPublicS3BucketsRole";
const policy_name: &str = "DetectPublicS3BucketsPolicy";
const policy_document: &str = r#"{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Action": [
                "s3:ListAllMyBuckets",
                "s3:GetBucketAcl"
            ],
            "Resource": "*"
        }
    ]
}
"#;
const assume_role_policy_document: &str = r#"{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Principal": {
                "Service": "s3.amazonaws.com"
            },
            "Action": "sts:AssumeRole"
        }
    ]
}
"#;
pub struct S3Analyzer {
    pub config: Arc<aws_config::SdkConfig>,
}
#[async_trait]
impl analyzer_trait::Analyzer for S3Analyzer {
    async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "{} {} {}",
            "Initialising".green(),
            "S3".blue(),
            "analyzer".green()
        );
       //  let c1 = Arc::clone(&self.config);
       //  // fetch account
       //  let acc = utils::sts::get_account_id(c1).await;
       //  println!("Account ID {}", acc.unwrap().green());
       //
       //  let c2 = Arc::clone(&self.config);
       //  let c3 = Arc::clone(&self.config);
       //  let c4 = Arc::clone(&self.config);
       //  let mut shouldCreate: bool = false;
       //  // Role ------------------------------------------------------------------------------------
       //  match utils::iam::check_role_exists(c2, role_name).await {
       //      Ok(x) => println!("Role {} exists", role_name),
       //      _ => {
       //          shouldCreate = true;
       //          println!("Role {} does not exist", role_name)
       //      }
       //  }
       //  if shouldCreate {
       //      let response = utils::iam::create_role(c3, role_name, assume_role_policy_document).await?;
       //      println!(
       //          "Created role {} with ARN {}",
       //          response.role_name.unwrap(),
       //          response.arn.unwrap()
       //      );
       //  }
       // // Policy ----------------------------------------------------------------------------------
       //  let policy = utils::iam::create_policy(c4, policy_name, policy_document).await;
       //  match policy {
       //      Ok(p) => println!("Created {:?}", p),
       //      Err(e) => {
       //          println!("Error creating policy {} {}", policy_name, e);
       //      },
       //  }
        // -----------------------------------------------------------------------------------------
        Ok(())
    }

    async fn de_init(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "{} {} {}",
            "De-initialising".green(),
            "S3".blue(),
            "analyzer".green()
        );

        Ok(())
    }
    async fn run(&self) -> Option<Vec<AnalysisResults>> {
        println!(
            "{} {} {}",
            "Running".green(),
            "S3".blue(),
            "analyzer".green()
        );
        let s3 = aws_sdk_s3::Client::new(&self.config);

        let s3_response = s3.list_buckets().send().await;

        for bucket in s3_response.unwrap().buckets {
            for b in bucket {
                let bucket_name = b.name.unwrap();
                // Check if the S3 bucket is publicly accessible.
                let acl_response = s3
                    .get_bucket_acl()
                    .bucket(&bucket_name)
                    .send()
                    .await;

                for grant in acl_response.unwrap().grants {
                    if let Some(grantee) = grant.first() {
                        if grantee.clone().grantee.unwrap().uri == Some("http://acs.amazonaws.com/groups/global/AllUsers".to_string()) {
                            println!("Publicly accessible S3 bucket: {}", bucket_name);
                        }
                    }
                }
            }
        }

        Some(vec![AnalysisResults {
            message: "".to_string(),
        }])
    }

    fn get_name(&self) -> &str {
        "s3"
    }
}
