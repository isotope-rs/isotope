
use seahorse::{Command, Context, Flag, FlagType, error::FlagError};
use crate::config;
use crate::config::Conf;

static CLOUD_TYPES: &[&str] = &["aws"];
pub fn add_commands() -> Command {
    Command::new("config")
        .description("Configure isotope")
        .alias("c")
        .usage("isotope config")
        .command(Command::new("set")
            .usage("config set")
            .description("isotope config set --cloud aws")
            .flag(
                Flag::new("cloud", FlagType::String)
                    .alias("cl"),
            )
            .action(run_configure))
        .command(Command::new("get")
            .description("isotope config get")
            .usage("config get")
            .action(show_configure))
}
fn show_configure(_context: &Context) {
    let c = config::get_or_create_config();
    match c {
        Ok(x) => println!("{:?}",x),
        Err(e) => println!("Error detected {:?}",e.to_string())
    }
}
fn run_configure(context: &Context) {
    let mut conf: Conf = config::Conf{ cloud:String::new()};
    let c = config::get_or_create_config();
    match c {
        Ok(x) => conf = x,
        Err(e) => println!("Error detected {:?}",e.to_string())
    }
    match context.string_flag("cloud") {
        Ok(cl) => if CLOUD_TYPES.contains(&cl.as_str()) {
            conf.cloud = cl.clone();
        }
        Err(e) => match e {
            FlagError::Undefined => println!("--cloud flag required"),
            FlagError::ArgumentError => panic!("argument error..."),
            FlagError::NotFound => panic!("flag not found..."),
            FlagError::ValueTypeError => panic!("value type mismatch..."),
            FlagError::TypeError => println!("Wrong flag type selected"),
        },
    }
    config::save_config(conf).unwrap();
}