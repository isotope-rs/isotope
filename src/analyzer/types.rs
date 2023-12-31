use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResults {
    pub message: String,
    pub analyzer_name: String,
    pub advice: String,
}

impl AnalysisResults {
    pub fn new() -> AnalysisResults {
        Self {
            message: "".to_string(),
            analyzer_name: "".to_string(),
            advice: "".to_string(),
        }
    }
}
