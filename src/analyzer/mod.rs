use crate::analyzer::analyzer_trait::Analyzer;

pub mod analyzer_trait;
pub mod s3_analyzer;
mod cloudwatch_analyzer;
#[derive(Debug,Clone,Copy)]
pub struct Results {

}
pub fn generate_analyzers( config: aws_config::SdkConfig,results: Vec<Results>) -> [Box<dyn Analyzer>; 2] {
    let analyzers: [Box<dyn Analyzer>; 2] = [Box::new(s3_analyzer::S3Analyzer{
        config: config.clone(),
        results: results.clone()
    }),
        Box::new(cloudwatch_analyzer::CloudwatchAnalyzer{
            config: config.clone(),
            results: results.clone()
        })
    ];

    return analyzers
}