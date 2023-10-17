use std::sync::{Arc, Mutex};
use crate::{analyzer, Args};

use crate::config;
use crate::config::Conf;


use aws_config::meta::region::RegionProviderChain;

use futures::StreamExt;
use crate::analyzer::analyzer_trait::Analyzer;
use crate::analyzer::types;
use crate::analyzer::types::AnalysisResults;


pub async fn run_analysis(args: &Args) {
    let mut conf: Conf = config::Conf{ cloud:String::new()};
    let c = config::get_or_create_config();
    match c {
        Ok(x) => conf = x,
        Err(e) => println!("Error detected {:?}",e.to_string())
    }
    // Setup available providers
    let region_provider = RegionProviderChain::default_provider();
    let config = aws_config::from_env().region(region_provider).load().await;

    // Create the results set
    let mut re: Vec<AnalysisResults> = vec!();


    let mut analyzers: Vec<Box<dyn Analyzer>> = analyzer::generate_analyzers(config.clone());

    match &args.Analyzer {
        Some(analyzerArg) => {
            let filteredAnalyzer = analyzers.iter().find(|x| x.get_name() == analyzerArg);
            match filteredAnalyzer {
                Some(x) => {
                    let response = x.run().await;
                    println!("{:?}", response);
                    match response {
                        Some(mut respResults) => {
                            re.append(&mut respResults);
                        },
                        None => { }
                    }
                },
                None => println!("Analyzer of type not found")
            }
        }
        None => {
            let tasks = analyzers.into_iter().map(|an: Box<dyn analyzer::analyzer_trait::Analyzer>|
                tokio::spawn(async move {
                     an.run().await;
                })).collect::<Vec<_>>();

            for task in tasks {
                task.await.unwrap();
            }
        }
    }

}
