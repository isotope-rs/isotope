use colored::Colorize;
use crate::analyzer::types::AnalysisResults;

pub struct Processor {
    analysis_results: Vec<String>,
    config: Option<Configuration>,
}

pub struct Configuration {
    pub json_output: bool,
}

impl Processor {
    pub fn new(analysis_results: Vec<String>, config: Option<Configuration>) -> Self {
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
        for elem in self.analysis_results.iter().filter(|&x| !x.is_empty()) {
            println!("{}:{}", elem.blue(),elem.green());
        }
    }

    fn print_json(&mut self) {
        self.analysis_results.retain(|x| !x.is_empty());
        println!("{}", serde_json::to_string_pretty(&self.analysis_results).unwrap());
    }
}

impl Configuration {
    pub fn new(json_output: bool) -> Self {
        Self { json_output }
    }
}
