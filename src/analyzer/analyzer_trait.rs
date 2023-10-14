use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use crate::analyzer;

#[async_trait]
pub trait Analyzer:  Sync + Send {
    async fn run(&self,config: &aws_config::SdkConfig, results: &Vec<analyzer::Results>);
}