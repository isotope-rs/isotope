use crate::config::{get_or_create_config, Conf};
use base64::engine::general_purpose;
use base64::Engine;
use colored::Colorize;
use std::collections::HashMap;
use std::error::Error;

pub async fn list() -> Result<(), Box<dyn Error>> {
    let mut conf: Conf = Conf {
        cloud: String::new(),
        stored_advice: HashMap::new(),
    };

    if let Ok(c) = get_or_create_config() {
        conf = c
    }
    if conf.stored_advice.is_empty() {
        println!("Cache is empty!");
        return Ok(());
    }
    println!("Cache keys:");
    // print out the cache internals
    for kv in conf.stored_advice {
        match general_purpose::STANDARD.decode(kv.0) {
            Ok(x) => match String::from_utf8(x) {
                Ok(y) => {
                    println!("> {}", y.yellow());
                    match general_purpose::STANDARD.decode(kv.1) {
                        Ok(a) => match String::from_utf8(a) {
                            Ok(c) => {
                                println!("{}", c.blue());
                            }
                            Err(_e) => (),
                        },
                        _b => (),
                    }
                }
                Err(_e) => (),
            },
            Err(_e) => (),
        }
    }
    Ok(())
}
