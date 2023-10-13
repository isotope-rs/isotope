mod analyze;
mod configure;
mod config;
use seahorse::{App, Flag, FlagType};
use std::env;
mod analyzer;
// const
const CARGO_PKG_NAME: &str = "isotope";
const CARGO_PKG_DESCRIPTION: &str = "Isotope allows for the debugging of AWS services with AI";
const CARGO_PKG_AUTHORS: &str = "AlexsJones";
const CARGO_PKG_VERSION: &str = "0.1";


fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(CARGO_PKG_NAME)
        .author(CARGO_PKG_AUTHORS)
        .description(CARGO_PKG_DESCRIPTION)
        .version(CARGO_PKG_VERSION)
        .flag( Flag::new("cloud", FlagType::String)
               )
        .usage("cli [args]")
        .command(analyze::add_commands())
        .command(configure::add_commands());

    app.run(args);
}
