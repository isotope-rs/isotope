use crate::analyzer::analyzer_trait::Analyzer;
use crate::analyzer::types::AnalysisResults;
use crate::config::Conf;
use crate::{analyzer, Args};
use crate::{config, outputs};
use aws_config::meta::region::RegionProviderChain;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

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
    // Create channels
    let (tx, rx): (Sender<Vec<AnalysisResults>>, Receiver<Vec<AnalysisResults>>) = mpsc::channel();
    let mut analyzers: Vec<Box<dyn Analyzer>> = analyzer::generate_analyzers(config.clone());

    match &args.Analyzer {
        Some(analyzerArg) => {
            let filteredAnalyzer = analyzers.iter().find(|x| x.get_name() == analyzerArg);
            match filteredAnalyzer {
                Some(x) => {
                    let thread_tx = tx.clone();
                    let response = x.run().await;
                    match response {
                        Some(respResults) => {
                            thread_tx.send(respResults).unwrap();
                        }
                        None => {
                            thread_tx.send(vec![AnalysisResults::new()]).unwrap();
                        }
                    }
                }
                None => println!("Analyzer of type not found"),
            }
        }
        None => {
            let mut tasks = vec![];
            // Generate threads
            let mut count = 0;
            for currentAnalyzer in analyzers {
                let thread_tx = tx.clone();
                tasks.push(tokio::spawn(async move {
                    let response = currentAnalyzer.run().await;
                    match response {
                        Some(respResults) => {
                            thread_tx.send(respResults).unwrap();
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
            for n in 0..count {
                let rxResult = rx.recv();
                results.append(&mut rxResult.unwrap());
            }
            for task in tasks {
                task.await.unwrap();
            }

            let mut process: outputs::Processor;

            match args.JSON {
                Some(x) => {
                    let p = outputs::Processor::new(results, Some(outputs::Configuration::new(x)));
                    p.print();
                }
                None => {
                    let p = outputs::Processor::new(results, None);
                    p.print();
                }
            }
        }
    }
}
