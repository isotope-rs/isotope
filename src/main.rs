use clap::{Parser, Subcommand};
mod analyze;
mod analyzer;
mod bedrock;
mod cache;
mod config;
mod outputs;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}
enum List {
    Analyzers,
    Cache,
}
#[derive(Subcommand)]
enum Commands {
    /// Run AWS account analysis
    Analyze {
        #[arg(short, long, long_help = "Select a single analyzer")]
        analyzer: Option<String>,
        #[arg(short, long, long_help = "Enable debug logging")]
        debug: bool,
        #[arg(short, long, long_help = "Print out results in JSON format")]
        json: bool,
        #[arg(
            short,
            long,
            long_help = "Use Bedrock AI to assist in remediation of issues"
        )]
        explain: bool,
    },
    /// List resources by type
    List {
        #[arg(short, long, long_help = "Resource type to list e.g. analyzers/cache")]
        resource: String,
    },
}
#[tokio::main]
async fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Analyze {
            analyzer,
            debug: _,
            json,
            explain,
        }) => {
            analyze::run_analysis(analyzer, json, explain).await;
        }
        Some(Commands::List { resource }) => match resource.as_str() {
            "analyzers" => {
                analyze::list_analyzers().await;
            }
            "cache" => {
                cache::list().await;
            }
            _ => {}
        },
        None => {}
    }
}
