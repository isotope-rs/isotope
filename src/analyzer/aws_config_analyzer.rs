
use crate::analyzer::analyzer_trait;
use aws_sdk_config::types::ResourceType;
use aws_sdk_config::{Client, Error};

use async_trait::async_trait;
use crate::analyzer::types::AnalysisResults;
use colored::Colorize;

pub struct AWSConfigAnalyzer {
    pub config: aws_config::SdkConfig,
}
#[async_trait]
impl analyzer_trait::Analyzer for AWSConfigAnalyzer {
    async fn run(&self) -> Option<Vec<AnalysisResults>> {
        println!("{} {} {}","Running".green(),"aws-config".blue(),"analyzer".green());
        let _client = Client::new(&self.config);

     //   show_resources(true,&client).await?;
        None
    }
     fn get_name(&self)-> &str {
        "aws_config"
    }
}
async fn show_resources(verbose: bool, client: &Client) -> Result<(),  Error> {

    for value in ResourceType::values() {
        let parsed = ResourceType::from(*value);
        let resp = client
            .list_discovered_resources()
            .resource_type(parsed)
            .send()
            .await?;

        let resources = resp.resource_identifiers();

        if resources.is_some() && verbose {
            println!();
            println!("Resources of type {}:", value);
        }

        for resource in resources {
            println!(
                "  Resource ID: {:?}",resource,
            );
        }
    }

    println!();

    Ok(())
}