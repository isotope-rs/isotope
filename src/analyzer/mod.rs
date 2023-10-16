use crate::analyzer::analyzer_trait::Analyzer;

pub mod analyzer_trait;
mod macie_analyzer;
mod aws_config_analyzer;

#[derive(Debug,Clone,Copy)]
pub struct Results {

}
pub fn generate_analyzers( config: aws_config::SdkConfig,results: Vec<Results>) -> [Box<dyn Analyzer>; 2] {
    let analyzers: [Box<dyn Analyzer>; 2] = [Box::new(aws_config_analyzer::AWSConfigAnalyzer {
        config: config.clone(),
        results: results.clone()
    }),
        Box::new(macie_analyzer::MacieAnalyzer{
            config: config.clone(),
            results: results.clone()
        })
    ];

    return analyzers
}