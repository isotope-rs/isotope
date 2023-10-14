mod analyze;
mod configure;
mod config;


mod analyzer;
// const
#[warn(dead_code)]
const CARGO_PKG_NAME: &str = "isotope";
#[warn(dead_code)]
const CARGO_PKG_DESCRIPTION: &str = "Isotope allows for the debugging of AWS services with AI";
#[warn(dead_code)]
const CARGO_PKG_AUTHORS: &str = "AlexsJones";
#[warn(dead_code)]
const CARGO_PKG_VERSION: &str = "0.1";

#[tokio::main]
async fn main() {

    analyze::run_analysis().await;
}
