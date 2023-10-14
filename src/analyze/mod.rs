
use crate::analyzer;
use std::sync::{Arc, Mutex};
use crate::config;
use crate::config::Conf;
use futures::future;
use seahorse::{Command, Context,Flag, FlagType};
use aws_config::meta::region::RegionProviderChain;
use futures::executor::block_on;

pub async fn run_analysis() {
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
    let results: Vec<analyzer::Results> = (Vec::new());
    for an in  analyzer::get_analyzers().into_iter() {
        tokio::spawn(future::lazy(|_| an.run(&config,&results)));
    }

}
