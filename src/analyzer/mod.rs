use crate::analyzer::analyzer_trait::Analyzer;

mod analyzer_trait;
pub mod s3_analyzer;
mod cloudwatch_analyzer;
pub struct Results {

}
pub fn get_analyzers() -> [Box<dyn Analyzer>; 2] {
    let analyzers: [Box<dyn Analyzer>; 2] = [Box::new(s3_analyzer::S3Analyzer{}),
        Box::new(cloudwatch_analyzer::CloudwatchAnalyzer{})];

    return analyzers
}