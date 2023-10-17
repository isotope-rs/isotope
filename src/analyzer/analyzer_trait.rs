

use async_trait::async_trait;
use crate::analyzer::types::AnalysisResults;


#[async_trait]
pub trait Analyzer:  Sync + Send {
    async fn run(&self) -> Option<Vec<AnalysisResults>>;
    fn get_name(&self) -> &str;
}