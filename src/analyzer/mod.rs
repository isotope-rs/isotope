use crate::analyzer::analyzer_trait::Analyzer;
use std::sync::Arc;

pub mod analyzer_trait;
mod s3_analyzer;
pub(crate) mod types;

pub fn generate_analyzers<'a>(config: aws_config::SdkConfig) -> Vec<Box<dyn Analyzer + 'a>> {
    let analyzers: Vec<Box<dyn Analyzer>> = vec![Box::new(s3_analyzer::S3Analyzer {
        config: Arc::new(config),
    })];
    analyzers
}
