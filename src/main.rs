use clap::{Parser, Subcommand};
mod analyze;
mod analyzer;
mod config;
mod outputs;
mod utils;
mod bedrock;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    /// List analyzers
    List {

    },
    /// Run AWS account analysis
    Analyze {
        #[arg(short, long, long_help="Select a single analyzer")]
        analyzer: Option<String>,
        #[arg(short, long, long_help="Enable debug logging")]
        debug: bool,
        #[arg(short, long, long_help="Print out results in JSON format")]
        json: bool,
        #[arg(short,long, long_help="Use Bedrock AI to assist in remediation of issues")]
        explain: bool,
    },
}
#[tokio::main]
async fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Analyze { analyzer, debug: _, json, explain}) => {

            analyze::run_analysis(analyzer,json,explain).await;
        },
        Some(Commands::List {}) => {
            analyze::list_analyzers().await;
        },
        None => {

        }
    }

}
