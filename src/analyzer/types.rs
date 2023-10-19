#[derive(Debug,Clone)]
pub struct AnalysisResults {
    pub message: String,
}

impl AnalysisResults {
    pub fn new() -> AnalysisResults{
        return Self{ message: "".to_string()}
    }
}