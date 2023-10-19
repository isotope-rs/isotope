use crate::analyzer::types::AnalysisResults;

pub struct Processor {
	analysis_results: Vec<AnalysisResults>,
	config: Option<Configuration>
}

pub struct Configuration {
	pub json_output: bool,
}

impl Processor {
	pub fn new(analysis_results: Vec<AnalysisResults>, config: Option<Configuration> ) -> Self {
		 Self{  analysis_results, config, }
	}

	pub fn print(&self) {

	}
}

impl Configuration {
	pub fn new(json_output: bool) -> Self {
		 Self{ json_output}
	}
}