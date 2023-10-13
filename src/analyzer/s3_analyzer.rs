use crate::analyzer::analyzer_trait;
use async_trait::async_trait;
use colored::Colorize;
use std::sync::{Arc, Mutex};
use crate::analyzer;

pub struct S3Analyzer {

}
#[async_trait]
impl analyzer_trait::Analyzer for S3Analyzer {
    async fn run(&self,config: aws_config::SdkConfig,results: Arc<Mutex<Vec<analyzer::Results>>>) {
        println!("{} {} {}","Running".green(),"S3".blue(),"analyzer".green())

    }
}