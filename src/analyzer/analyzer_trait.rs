use async_trait::async_trait;

#[async_trait]
pub trait Analyzer {
    async fn run(&self);
}