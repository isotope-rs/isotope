

use crate::analyzer::analyzer_trait;

use async_trait::async_trait;
use crate::analyzer::types::AnalysisResults;
use colored::Colorize;


pub struct MacieAnalyzer {
    pub config: aws_config::SdkConfig,
}
#[async_trait]
impl analyzer_trait::Analyzer for MacieAnalyzer {
    async fn run(&self) -> Option<Vec<AnalysisResults>> {
        println!("{} {} {}","Running".green(),"macie".blue(),"analyzer".green());

        //TODO: This is demo code to show the results capture works
        Some(vec!(AnalysisResults{
            message: "Something broken".to_string()
        }) )
    }
     fn get_name(&self) -> &str {
        "macie"
    }
}