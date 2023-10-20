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
		match &self.config {
			Some(x)=> {
				match x.json_output {
					true => {
						self.print_json();
					},
					_ => self.print_text(),
				}
			},
			_ => self.print_text(),
		}

	}
	fn print_text(&self) {
		self.analysis_results.iter().for_each(|x| println!("{:?}",x.message));
	}

	fn print_json(&self) {

		let v = serde_json::to_value(&self.analysis_results).unwrap();
		print!("{:?}",v);
	}
}

impl Configuration {
	pub fn new(json_output: bool) -> Self {
		 Self{ json_output}
	}
}