use crate::analyzer::analyzer_trait::Analyzer;
use std::sync::Arc;

pub mod analyzer_trait;
mod ebs_analyzer;
mod rds_analyzer;
mod s3_analyzer;
mod sts_analyzer;
pub(crate) mod types;

pub fn generate_analyzers(config: aws_config::SdkConfig) -> Vec<Box<dyn Analyzer>> {
    vec![
        Box::new(s3_analyzer::S3Analyzer {
            config: config.clone(),
        }),
        Box::new(sts_analyzer::STSAnalyzer {
            config: config.clone(),
        }),
        Box::new(rds_analyzer::RDSAnalyzer {
            config: config.clone(),
        }),
        Box::new(ebs_analyzer::EbsAnalyzer {
            config: config.clone(),
        }),
    ]
}
