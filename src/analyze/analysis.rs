use std::sync::{Arc, Mutex};
use seahorse::{Context};
use crate::config::Conf;
use colored::Colorize;
use aws_config::SdkConfig;
use crate::analyzer;
pub struct CreateAnalysis<'a> {
    pub context: &'a Context,
    pub aws: SdkConfig,
    pub  conf: Conf
}

impl<'a> CreateAnalysis<'a> {
    pub async fn run(self, _results: Arc<Mutex<Vec<analyzer::Results>>>) {
        println!("{}:{}","Using cloud provider ".green(),self.conf.cloud.blue());
       for an in  analyzer::get_analyzers().into_iter() {
            an.run().await
       }
    }
}

