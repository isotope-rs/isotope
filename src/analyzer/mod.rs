
use crate::analyzer::analyzer_trait::Analyzer;

pub mod analyzer_trait;
mod macie_analyzer;
mod aws_config_analyzer;
pub(crate) mod types;

pub fn generate_analyzers<'a>( config: aws_config::SdkConfig) -> Vec<Box<dyn Analyzer  + 'a>> {
    let analyzers: Vec<Box<dyn Analyzer>> = vec!(Box::new(aws_config_analyzer::AWSConfigAnalyzer {
        config: config.clone(),
    }),
        Box::new(macie_analyzer::MacieAnalyzer{
            config: config.clone(),
        })
    );
    return analyzers
}