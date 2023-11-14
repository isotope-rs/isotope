use std::env;
use crate::analyzer::analyzer_trait;
use crate::analyzer::analyzer_trait::Analyzer;
use crate::analyzer::types::AnalysisResults;
use async_trait::async_trait;
use aws_sdk_rds;
use aws_types::region::Region;


pub struct RDSAnalyzer {

}
#[async_trait]
impl analyzer_trait::Analyzer for RDSAnalyzer {
    async fn run(&self) -> Option<Vec<AnalysisResults>> {
        let mut results = Vec::new();
        let aws_region = env::var("AWS_REGION").unwrap();
        let region = Region::new(aws_region);
        let config = aws_config::from_env().region(region).load().await;
        let rds = aws_sdk_rds::Client::new(&config);
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
#[tokio::test]
async fn get_name_test() {
    let rds_analyzer = RDSAnalyzer {
    };
    assert_eq!(rds_analyzer.get_name(), "rds".to_string());
}
