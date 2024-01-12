use clap::{Parser, Subcommand};
use std::env;
use std::process::exit;
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
        #[arg(short, long, long_help = "Enable interactive conversational mode")]
        interactive: bool,
    },
    /// List resources by type
    List {
        #[arg(short, long, long_help = "Resource type to list e.g. analyzers/cache")]
        resource: String,
    },
}

const ENVS: &'static [&'static str] = &[
    "BEDROCK_REGION",
    "BEDROCK_MODEL",
    "AWS_REGION",
    "AWS_ACCESS_KEY",
    "AWS_SECRET_ACCESS_KEY",
];

#[tokio::main]
async fn main() {
    env_logger::init();

    for e in ENVS {
        let r = env::var(e);
        match r {
            Ok(x) => {}
            Err(_e) => {
                println!("ENV: {} not found", e);
                exit(1);
            }
        }
    }

    let args = Args::parse();
    match &args.command {
        Some(Commands::Analyze {
            analyzer,
            debug: _,
            json,
            explain,
            interactive,
        }) => {
            #[warn(unused_must_use)]
            analyze::run_analysis(analyzer, json, explain, interactive).await;
        }
        Some(Commands::List { resource }) => match resource.as_str() {
            "analyzers" => {
                #[warn(unused_must_use)]
                analyze::list_analyzers().await;
            }
            "cache" => {
                #[warn(unused_must_use)]
                cache::list().await;
            }
            _ => {}
        },
        None => {}
    }
}
