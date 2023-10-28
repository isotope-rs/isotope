use crate::analyzer::types::AnalysisResults;
use async_trait::async_trait;

#[async_trait]
pub trait Analyzer: Sync + Send {
    async fn run(&self) -> Option<Vec<AnalysisResults>>;
    fn get_name(&self) -> &str;
}
