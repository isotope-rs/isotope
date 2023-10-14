
use async_trait::async_trait;


#[async_trait]
pub trait Analyzer:  Sync + Send {
    async fn run(&self);
}