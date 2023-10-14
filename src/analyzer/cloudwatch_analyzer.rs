use crate::analyzer::analyzer_trait;
use async_trait::async_trait;
use colored::Colorize;

use crate::analyzer;

pub struct CloudwatchAnalyzer {
    pub config: aws_config::SdkConfig,
    pub results:  Vec<analyzer::Results>,
}
#[async_trait]
impl<'a> analyzer_trait::Analyzer for CloudwatchAnalyzer {
    async fn run(&self) {
        println!("{} {} {}","Running".green(),"Cloudwatch".blue(),"analyzer".green())

    }
}