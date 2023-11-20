use crate::analyzer::analyzer_trait;
use crate::analyzer::analyzer_trait::Analyzer;
use crate::analyzer::types::AnalysisResults;
use crate::utils;
use async_trait::async_trait;
use aws_sdk_rds;
use aws_types::sdk_config::SdkConfig;

pub struct RDSAnalyzer {
    pub config: SdkConfig,
}
#[async_trait]
impl analyzer_trait::Analyzer for RDSAnalyzer {
    async fn run(&self) -> Option<Vec<AnalysisResults>> {
        let mut results = Vec::new();
        let rds = aws_sdk_rds::Client::new(&self.config);
        let response = rds.describe_db_instances().send().await;
        for dbinstances in response {
            for vdbs in dbinstances.db_instances.iter() {
                for dbs in vdbs.iter() {
                    match dbs.publicly_accessible {
                        Some(_x) => results.push(AnalysisResults {
                            message: format!(
                                "Publicly accessible RDS instance {}",
                                dbs.db_instance_identifier.clone().unwrap()
                            ),
                            advice: "".to_string(),
                            analyzer_name: self.get_name(),
                        }),
                        None => (),
                    }
                }
            }
        }

        Some(results)
    }

    fn get_name(&self) -> String {
        "rds".to_string()
    }
}
