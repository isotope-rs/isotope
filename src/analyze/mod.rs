mod analysis;
use crate::analyzer;
use std::sync::{Arc, Mutex};
use crate::config;
use crate::config::Conf;
use seahorse::{Command, Context,Flag, FlagType};
use aws_config::meta::region::RegionProviderChain;
use futures::executor::block_on;
pub fn add_commands() -> Command {
    Command::new("analyze")
        .description("Analyze a connected cloud account")
        .alias("a")
        .usage("isotope analyze")
        .flag(
            Flag::new("explain", FlagType::Bool)
                .description("Sends data to remote AI for parsing")
                .alias("e"),
        )
        .action(run_analysis)
}
fn run_analysis(cont: &Context) {
    let mut conf: Conf = config::Conf{ cloud:String::new()};
    let c = config::get_or_create_config();
    match c {
        Ok(x) => conf = x,
        Err(e) => println!("Error detected {:?}",e.to_string())
    }
    // Setup available providers
    let region_provider = RegionProviderChain::default_provider().
        or_else("eu-west-2");
    let config = aws_config::from_env().region(region_provider).load();
    let r = block_on(config);
    let analysis = analysis::CreateAnalysis{
        context: cont,
    aws: r, conf: conf};

    // Create the results set
    let mut results: Arc<Mutex<Vec<analyzer::Results>>> = Arc::new((Mutex::new(Vec::new())));

    block_on(analysis.run(results))
}
