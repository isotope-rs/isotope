use crate::analyzer::analyzer_trait::Analyzer;
use crate::analyzer::types::AnalysisResults;
use crate::config::Conf;
use crate::{analyzer, Args, bedrock};
use crate::{config, outputs};
use aws_config::meta::region::{ProvideRegion, RegionProviderChain};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use colored::Colorize;

pub async fn run_analysis(args: &Args) {
    let mut conf: Conf = config::Conf {
        cloud: String::new(),
    };
    let c = config::get_or_create_config();
    match c {
        Ok(x) => conf = x,
        Err(e) => println!("Error detected {:?}", e.to_string()),
    }
    // Setup available providers
    let region_provider = RegionProviderChain::default_provider();
    let config = aws_config::from_env().region(region_provider).load().await;
    // Setup bedrock
    let bedrockClient = bedrock::BedrockClient::new(config.clone());

    println!("Current AWS region: {}", RegionProviderChain::default_provider().region().await.unwrap().as_ref().yellow());
    // Create channels
    let (tx, rx): (Sender<Vec<AnalysisResults>>, Receiver<Vec<AnalysisResults>>) = mpsc::channel();
    let analyzers: Vec<Box<dyn Analyzer>> = analyzer::generate_analyzers(config.clone());

    match &args.analyzer {
        Some(analyzer_arg) => {
            let filtered_analyzer = analyzers.iter().find(|x| x.get_name().as_str() == analyzer_arg);
            match filtered_analyzer {
                Some(x) => {
                    let thread_tx = tx.clone();

                    let response = x.run().await;
                    match response {
                        Some(resp_results) => {
                            thread_tx.send(resp_results).unwrap();
                        }
                        None => {
                            thread_tx.send(vec![AnalysisResults::new()]).unwrap();
                        }
                    }
                }
                None => println!("analyzer of type not found"),
            }
        }
        None => {
            let mut tasks = vec![];
            // Generate threads
            let mut count = 0;
            for current_analyzer in analyzers {
                let thread_tx = tx.clone();
                tasks.push(tokio::spawn(async move {
                    let response = current_analyzer.run().await;
                    match response {
                        Some(resp_results) => {
                            thread_tx.send(resp_results).unwrap();
                        }
                        None => {
                            thread_tx.send(vec![AnalysisResults::new()]).unwrap();
                        }
                    }
                }));
                count += 1;
            }
            let mut results: Vec<AnalysisResults> = vec![];
            // Aggregate results
            for _n in 0..count {
                let rx_result = rx.recv();
                results.append(&mut rx_result.unwrap());
            }
            for task in tasks {
                task.await.unwrap();
            }

            let mut processed_results: Vec<String> = vec![];
            // Feed results into Bedrock
            for res in results.clone() {
                if !res.message.is_empty() {
                    let result = bedrockClient.enrich(res.message).await;
                    match result {
                        Ok(x) => (
                            processed_results.push(x)
                        ),
                        Err(e) => println!("{}", e)
                    }
                }
            }
            //
            match args.json {
                Some(x) => {
                    let mut p = outputs::Processor::new(processed_results, Some(outputs::Configuration::new(x)));
                    p.print();
                }
                None => {
                    let mut p = outputs::Processor::new(processed_results, None);
                    p.print();
                }
            }
        }
    }
}
