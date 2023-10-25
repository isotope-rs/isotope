

use async_trait::async_trait;
use crate::analyzer::types::AnalysisResults;

#[async_trait]
pub trait Analyzer:  Sync + Send {
    async fn run(&self) -> Option<Vec<AnalysisResults>>;

    async fn init(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn de_init(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn get_name(&self) -> &str;
}