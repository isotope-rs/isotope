use crate::analyzer::analyzer_trait::Analyzer;


pub mod analyzer_trait;
mod ebs_analyzer;
mod rds_analyzer;
mod s3_analyzer;
mod sts_analyzer;
pub(crate) mod types;
mod sg_analyzer;

pub fn generate_analyzers() -> Vec<Box<dyn Analyzer>> {
    let analyzers: Vec<Box<dyn Analyzer>> = vec![
        Box::new(s3_analyzer::S3Analyzer {
        }),
        Box::new(sts_analyzer::STSAnalyzer {
        }),
        Box::new(rds_analyzer::RDSAnalyzer {
        }),
        Box::new(ebs_analyzer::EbsAnalyzer {
        }),
        Box::new(sg_analyzer::SecurityGroupsAnalyzer {
        })
    ];
    analyzers
}
