use crate::analyzer::types::AnalysisResults;
use async_trait::async_trait;

#[async_trait]
pub trait Analyzer: Sync + Send {
    async fn run(&self) -> Option<Vec<AnalysisResults>>;
    fn get_name(&self) -> String;
}


#[tokio::test]
async fn build_analyzer_test() {
    struct TestAnalyzer {};
    #[async_trait]
    impl Analyzer for TestAnalyzer {
         fn get_name(&self) -> String {
            return "test".to_string()
        }
        async fn run(&self) -> Option<Vec<AnalysisResults>> {
            Some(vec![AnalysisResults{
                message: "".to_string(),
                analyzer_name: self.get_name(),
                advice: "".to_string()
            }])
        }
    }
    let test_analyzer = TestAnalyzer {

    };
    assert_eq!(test_analyzer.get_name(), "test".to_string());
    assert_eq!(test_analyzer.run().await.unwrap().is_empty(), false);
}
