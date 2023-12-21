use crate::analyzer::analyzer_trait::Analyzer;
use aws_types::sdk_config::SdkConfig;

pub mod analyzer_trait;
mod ebs_analyzer;
mod ec2_snapshot_analyzer;
mod eip_analzyer;
mod rds_analyzer;
mod s3_analyzer;
mod sg_analyzer;
mod sts_analyzer;
pub(crate) mod types;
mod iam_analyzer;

pub fn generate_analyzers(config: &SdkConfig) -> Vec<Box<dyn Analyzer>> {
    let analyzers: Vec<Box<dyn Analyzer>> = vec![
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
        Box::new(sg_analyzer::SecurityGroupsAnalyzer {
            config: config.clone(),
        }),
        Box::new(eip_analzyer::EipAnalyzer {
            config: config.clone(),
        }),
        Box::new(ec2_snapshot_analyzer::EC2SnapshotAnalyzer {
            config: config.clone(),
        }),
        Box::new(iam_analyzer::IamAnalyzer {
            config: config.clone(),
        })
    ];
    analyzers
}
