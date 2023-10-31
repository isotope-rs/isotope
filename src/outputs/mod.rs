use std::collections::HashMap;
use colored::Colorize;
use crate::analyzer::types::AnalysisResults;

pub struct Processor {
    analysis_results: HashMap<String,Vec<AnalysisResults>>,
    config: Option<Configuration>,
}

pub struct Configuration {
    pub json_output: bool,
}

impl Processor {
    pub fn new(analysis_results: HashMap<String,Vec<AnalysisResults>>, config: Option<Configuration>) -> Self {
        Self {
            analysis_results,
            config,
        }
    }

    pub fn print(&mut self) {
        match &self.config {
            Some(x) => match x.json_output {
                true => {
                    self.print_json();
                }
                _ => self.print_text(),
            },
            _ => self.print_text(),
        }
    }
    fn print_text(&self) {
        for (key, value) in &self.analysis_results {
            for results in value.iter() {
                println!("{}",results.message.blue());
                println!("{}",results.advice.green())
            }
        }
    }

    fn print_json(&mut self) {
        println!("{}", serde_json::to_string_pretty(&self.analysis_results).unwrap());
    }
}

impl Configuration {
    pub fn new(json_output: bool) -> Self {
        Self { json_output }
    }
}
