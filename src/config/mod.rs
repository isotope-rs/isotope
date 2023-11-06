mod cache;

use serde::{Deserialize, Serialize};
use simple_home_dir::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub const CONFFILE: &str = ".isotope.config";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conf {
    pub cloud: String,
    // This is stored as a hashed string representing the issue e.g. S3 bucket public
    // With a subsequent value (also b64 encoded)
    pub stored_advice: HashMap<String, String>,
}
impl Conf {
    pub fn new() -> Self {
        Self {
            cloud: "".to_string(),
            stored_advice: Default::default(),
        }
    }
}
pub fn get_conf_path() -> String {
    let home = home_dir().unwrap();
    let mut confpath = home.to_str().unwrap().to_string();
    confpath.push('/');
    confpath.push_str(CONFFILE);
    confpath
}
pub fn save_config(config: &Conf) -> Result<(), Box<dyn Error>> {
    let p = get_conf_path();
    let s = serde_json::to_string(&config)?;
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(p.as_str())?;
    f.write_all(s.as_bytes())?;
    f.flush()?;
    Ok(())
}
pub fn get_or_create_config() -> Result<Conf, Box<dyn Error>> {
    let p = get_conf_path();
    let c = Conf {
        cloud: String::new(),
        stored_advice: HashMap::new(),
    };
    if !Path::new(&p).exists() {
        let mut f = File::create(&p)?;
        let s = serde_json::to_string(&c)?;
        f.write_all(s.as_bytes())?;
    } else {
        let mut f = File::open(p)?;
        let mut data = String::new();
        f.read_to_string(&mut data)?;
        let loaded_config: Conf = serde_json::from_str(data.as_str())?;
        return Ok(loaded_config);
    }
    Ok(c)
}
#[test]
fn config_test() {
    let p = get_conf_path();
    assert_ne!(p.len(), 0);
    assert_eq!(p.contains(CONFFILE), true);
}

#[test]

fn write_read_config_test() {
    let k = "gamma";
    let v = "epsilon";
    let mut conf: Conf = Conf::new();
    if let Ok(c) = get_or_create_config() {
        conf = c
    }
    conf = conf.upsert_into_cache(k, v);
    match conf.fetch_from_cache(k) {
        Some(x) => {
            let _ = conf.remove_from_cache(k);
            assert_eq!(x, v);
        }
        None => {
            let _ = conf.remove_from_cache(k);
            assert_ne!(0, 0);
        }
    }
}
