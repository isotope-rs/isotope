use crate::analyzer::analyzer_trait;
use async_trait::async_trait;
use colored::Colorize;
pub struct S3Analyzer {

}
#[async_trait]
impl analyzer_trait::Analyzer for S3Analyzer {
    async fn run(&self) {
        println!("{} {} {}","Running".green(),"S3".blue(),"analyzer".green())
    }
}