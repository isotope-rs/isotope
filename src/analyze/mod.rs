
use crate::{analyzer, Args};

use crate::config;
use crate::config::Conf;


use aws_config::meta::region::RegionProviderChain;
use colored::Colorize;
use futures::StreamExt;
use crate::analyzer::analyzer_trait::Analyzer;


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
    let results: Vec<analyzer::Results> = Vec::new();
    let analyzers: [Box<dyn analyzer::analyzer_trait::Analyzer>; 2] = analyzer::generate_analyzers(config.clone(), results.clone());

    if args.Analyzer != "" {
        let filteredAnalyzer = analyzers.iter().find(|x| x.get_name() == args.Analyzer.as_str());
        match filteredAnalyzer {
            Some(x) => {
                x.run().await
            },
            None => { println!("Analyzer of type {} not found", args.Analyzer.as_str().red())},
        }
    }else {

        let tasks = analyzers.into_iter().map(|an: Box<dyn analyzer::analyzer_trait::Analyzer>| tokio::spawn(async move {
            an.run().await;
        })).collect::<Vec<_>>();

        for task in tasks {
            task.await.unwrap();
        }
    }
}
