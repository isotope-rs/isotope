use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use crate::analyzer;

#[async_trait]
pub trait Analyzer {
    async fn run(&self,config: aws_config::SdkConfig, results: Arc<Mutex<Vec<analyzer::Results>>>);
}