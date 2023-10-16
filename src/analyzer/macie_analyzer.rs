
use crate::analyzer;
use crate::analyzer::analyzer_trait;
use async_trait::async_trait;
use colored::Colorize;
use aws_sdk_macie;

pub struct MacieAnalyzer {
    pub config: aws_config::SdkConfig,
    pub results:  Vec<analyzer::Results>,
}
#[async_trait]
impl<'a> analyzer_trait::Analyzer for MacieAnalyzer {
    async fn run(&self) {
        println!("{} {} {}","Running".green(),"macie".blue(),"analyzer".green());
        // let client = aws_sdk_macie::Client::new(&self.config);
        //
        // let result = client.associate_member_account()
        //     .member_account_id("493588633530")
        //     .send()
        //     .await;
        //
        // println!("{:?}",result);
    }
     fn get_name(&self) -> &str {
        "macie"
    }
}