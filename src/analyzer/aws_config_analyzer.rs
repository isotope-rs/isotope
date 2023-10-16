use std::any::Any;
use crate::analyzer;
use crate::analyzer::analyzer_trait;
use aws_sdk_config::types::ResourceType;
use aws_sdk_config::{config::Region, meta::PKG_VERSION, Client, Error};
use async_trait::async_trait;
use colored::Colorize;

pub struct AWSConfigAnalyzer {
    pub config: aws_config::SdkConfig,
    pub results:  Vec<analyzer::Results>,
}
#[async_trait]
impl<'a> analyzer_trait::Analyzer for AWSConfigAnalyzer {
    async fn run(&self) {
        println!("{} {} {}","Running".green(),"aws-config".blue(),"analyzer".green());
        let client = Client::new(&self.config);

        show_resources(true,&client).await;
    }
     fn get_name(&self)-> &str {
        "aws_config"
    }
}
async fn show_resources(verbose: bool, client: &Client) -> Result<(), Error> {

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