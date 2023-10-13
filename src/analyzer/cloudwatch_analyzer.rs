use crate::analyzer::analyzer_trait;
use async_trait::async_trait;
use colored::Colorize;

pub struct CloudwatchAnalyzer {

}
#[async_trait]
impl analyzer_trait::Analyzer for CloudwatchAnalyzer {
    async fn run(&self) {
        println!("{} {} {}","Running".green(),"Cloudwatch".blue(),"analyzer".green())
    }
}