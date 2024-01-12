use crate::analyzer::types::AnalysisResults;
use colored::Colorize;
use std::collections::HashMap;

pub struct Processor {
    analysis_results: HashMap<String, Vec<AnalysisResults>>,
    config: Option<Configuration>,
    explain: bool,
}

pub struct Configuration {
    pub json_output: bool,
}

impl Processor {
    pub fn new(
        analysis_results: HashMap<String, Vec<AnalysisResults>>,
        config: Option<Configuration>,
        explain: bool,
    ) -> Self {
        Self {
            analysis_results,
            config,
            explain,
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
        self.analysis_results.iter().for_each(|(_key, value)| {
            for results in value.iter() {
                println!("{}", results.message.blue());
                if self.explain {
                    println!("{}", results.advice.green())
                }
            }
        });
    }

    fn print_json(&mut self) {
        println!(
            "{}",
            serde_json::to_string_pretty(&self.analysis_results).unwrap()
        );
    }
}

impl Configuration {
    pub fn new(json_output: bool) -> Self {
        Self { json_output }
    }
}
