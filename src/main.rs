use clap::{Parser};
mod analyze;
mod configure;
mod config;
mod analyzer;
mod outputs;

// const
const CARGO_PKG_NAME: &str = "isotope";
const CARGO_PKG_DESCRIPTION: &str = "Isotope allows for the debugging of AWS services with AI";
const CARGO_PKG_AUTHORS: &str = "AlexsJones";
const CARGO_PKG_VERSION: &str = "0.1";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    analyzer: Option<String>,
    #[arg(short, long)]
    verbose: bool,
    #[arg(short,long)]
    json: Option<bool>,
}
#[tokio::main]
async fn main() {

    let args = Args::parse();

    analyze::run_analysis(&args).await;
}
