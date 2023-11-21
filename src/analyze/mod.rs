use crate::analyzer::analyzer_trait::Analyzer;
use crate::analyzer::types::AnalysisResults;
use crate::config::Conf;
use crate::{analyzer, bedrock, utils};
use crate::{config, outputs};
use aws_config::meta::region::RegionProviderChain;
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

pub async fn list_analyzers() -> Result<(), Box<dyn Error>> {
    // Setup available providers
    let _region_provider = RegionProviderChain::default_provider();
    let config = utils::load_config().await;
    let analyzers: Vec<Box<dyn Analyzer>> = analyzer::generate_analyzers(&config);
    println!("Analyzers");
    for analyzer in analyzers {
        println!("> {}", analyzer.get_name());
    }
    Ok(())
}
pub async fn run_analysis(
    selected_analyzer: &Option<String>,
    enable_json: &bool,
    explain: &bool,
) -> Result<(), Box<dyn Error>> {
    // TODO: Refactor this horrible initialisation of the config
    let mut conf: Conf = Conf::new();
    if let Ok(c) = config::get_or_create_config() {
        conf = c
    }

    // The first action should be to establish if configuration is valid
    let config = utils::load_config().await;

    // Setup bedrock
    let bedrock_client = bedrock::BedrockClient::new();
    // Create channels
    let (tx, rx): (Sender<Vec<AnalysisResults>>, Receiver<Vec<AnalysisResults>>) = mpsc::channel();

    let m = MultiProgress::new();
    let mut tasks = vec![];
    let mut count = 0;

    match selected_analyzer {
        Some(analyzer_arg) => {
            let filtered_analyzer = analyzer::generate_analyzers(&config)
                .into_iter()
                .find(|x| x.get_name() == *analyzer_arg);

            match filtered_analyzer {
                Some(x) => {
                    let thread_tx = tx.clone();
                    let pb = m.add(ProgressBar::new(count));
                    pb.enable_steady_tick(Duration::from_millis(200));
                    pb.set_style(
                        ProgressStyle::with_template("{prefix:.dim.bold} {spinner} {wide_msg}")
                            .unwrap()
                            .tick_chars("/|\\- "),
                    );
                    pb.set_prefix(format!("[{}/{}]", count + 1, 1));
                    pb.set_message(format!("Starting {} analyzer", x.get_name()));
                    tasks.push(tokio::spawn(async move {
                        let response = x.run().await;
                        match response {
                            Some(resp_results) => {
                                tx.send(resp_results).unwrap();
                                pb.finish_with_message("done...");
                            }
                            None => {
                                thread_tx.send(vec![AnalysisResults::new()]).unwrap();
                                pb.finish_with_message("done...");
                            }
                        }
                    }));
                    count = count + 1;
                }
                None => println!("analyzer of type not found"),
            }
        }
        None => {
            let analyzers: Vec<Box<dyn Analyzer>> = analyzer::generate_analyzers(&config);
            // Generate threads
            let alen = analyzers.len();
            for current_analyzer in analyzers {
                let pb = m.add(ProgressBar::new(count));
                pb.enable_steady_tick(Duration::from_millis(200));
                pb.set_style(
                    ProgressStyle::with_template("{prefix:.dim.bold} {spinner} {wide_msg}")
                        .unwrap()
                        .tick_chars("/|\\- "),
                );

                pb.set_prefix(format!("[{}/{}]", count + 1, alen));
                pb.set_message(format!("Starting {} analyzer", current_analyzer.get_name()));
                count = count + 1;
                let thread_tx = tx.clone();
                tasks.push(tokio::spawn(async move {
                    let response = current_analyzer.run().await;

                    match response {
                        Some(resp_results) => {
                            thread_tx.send(resp_results).unwrap();
                            pb.finish_with_message("done...");
                        }
                        None => {
                            thread_tx.send(vec![AnalysisResults::new()]).unwrap();
                            pb.finish_with_message("done...");
                        }
                    }
                }));
            }
        }
    }

    let mut results: Vec<AnalysisResults> = vec![];
    // Aggregate results
    for _n in 0..tasks.len() {
        let rx_result = rx.recv();
        results.append(&mut rx_result.unwrap());
    }
    for task in tasks {
        task.await.unwrap();
    }
    m.clear().unwrap();

    let mut processed_results: HashMap<String, Vec<AnalysisResults>> = HashMap::new();
    // generate Vectors aligned to each analyzer type
    // Feed results into Bedrock
    for mut res in results {
        if !res.message.is_empty() {
            // Check if the data is in the cache
            match conf.fetch_from_cache(&res.message) {
                Some(x) => res.advice = x.clone(),
                None => {
                    let result = bedrock_client.enrich(res.message.clone()).await;
                    // TODO: missing step to copy the bedrock result into res
                    match result {
                        Ok(x) => {
                            res.advice = x.clone();
                            // upsert into the cache for next time
                            conf.clone().upsert_into_cache(&res.message, &x);
                            // pass ownership over of advice
                            // check if the processed results analyzer exists as key
                            // upsert the analysis result into the vector
                        }
                        Err(_e) => (),
                    }
                }
            }
            match processed_results.entry(res.analyzer_name.clone()) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(res);
                }
                Entry::Vacant(e) => {
                    e.insert(vec![res]);
                }
            }
        }
    }

    if *enable_json {
        let mut p = outputs::Processor::new(
            processed_results,
            Some(outputs::Configuration::new(*enable_json)),
            *explain,
        );
        p.print();
    } else {
        let mut p = outputs::Processor::new(processed_results, None, *explain);
        p.print();
    }
    Ok(())
}
