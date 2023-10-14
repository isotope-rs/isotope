mod analyze;
mod configure;
mod config;


mod analyzer;
// const
const CARGO_PKG_NAME: &str = "isotope";
const CARGO_PKG_DESCRIPTION: &str = "Isotope allows for the debugging of AWS services with AI";
const CARGO_PKG_AUTHORS: &str = "AlexsJones";
const CARGO_PKG_VERSION: &str = "0.1";

#[tokio::main]
async fn main() {

    analyze::run_analysis().await;
}
