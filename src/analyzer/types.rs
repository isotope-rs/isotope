#[derive(Debug,Clone)]
pub struct AnalysisResults {
    pub Message: String,
}

impl AnalysisResults {
    pub fn new() -> AnalysisResults{
        return Self{Message: "".to_string()}
    }
}