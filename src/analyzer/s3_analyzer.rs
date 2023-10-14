use crate::analyzer::analyzer_trait;
use async_trait::async_trait;
use colored::Colorize;

use crate::analyzer;
use aws_sdk_s3::{Client};
pub struct S3Analyzer {
    pub config: aws_config::SdkConfig,
    pub results:  Vec<analyzer::Results>,
}
#[async_trait]
impl analyzer_trait::Analyzer for S3Analyzer  {
    async fn run(&self) {
        println!("{} {} {}","Running".green(),"S3".blue(),"analyzer".green());
        let client = Client::new(&self.config);
        let resp = client.list_buckets().send().await;
        match resp {
            Ok(x) => {
                for bucket in x.buckets {

                   for b in bucket.into_iter() {
                       println!("Found bucket {}", b.name.unwrap());
                   }
                }
            },
            Err(x) => {
                println!("Something went wrong:{}", x.to_string())
            }
        }
    }
}