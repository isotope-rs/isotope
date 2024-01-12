use std::error::Error;
use std::io::Write;

use crate::config::{save_config, Conf, get_conf_path};
use base64::{engine::general_purpose, Engine as _};
use log::debug;

impl Conf {
    fn encode_cache_key(&self, raw_cache_key: &str) -> String {
        general_purpose::STANDARD.encode(raw_cache_key)
    }
    pub fn save_config(&mut self) -> Result<(), Box<dyn Error>> {
        let p = get_conf_path();
        let s = serde_json::to_string(&self)?;
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(p.as_str())?;
        f.write_all(s.as_bytes())?;
        f.flush()?;
        Ok(())
    }
    pub fn upsert_into_cache(&mut self, raw_cache_key: &str, payload: &str) {
        let encoded_key = self.encode_cache_key(raw_cache_key);
        debug!("converted raw key {} into encoded cache key {}", &raw_cache_key, &encoded_key);
        let encoded_payload = self.encode_cache_key(payload);
        // append the new data to the hashmap
        self.stored_advice.insert(encoded_key.clone(), encoded_payload);

        debug!("upserting encoded cache key {}", &encoded_key);
        // In this scenario we cannot use ? operand as it impacts creation of the struct
        self.save_config().unwrap()
    }
    pub fn remove_from_cache(mut self, raw_cache_key: &str) -> Result<Self, Box<dyn Error>> {
        // let encoded_key = self.encode_cache_key(raw_cache_key);
        // self.stored_advice.remove(encoded_key.as_str());
        // self.save_config(self)?
        panic!()
    }
    pub fn fetch_from_cache(&self, raw_cache_key: &str) -> Option<String> {
        let encoded_key = self.encode_cache_key(raw_cache_key);
        let found_keys: Vec<&String> = self
            .stored_advice
            .iter()
            .filter_map(|(k, v)| {
                if k == encoded_key.as_str() {
                    Some(v)
                } else {
                    None
                }
            })
            .collect();

        // TODO: verify this
        if found_keys.is_empty() {
            debug!("Cache miss {}", &encoded_key);
            return None;
        }

        let mut found = String::new();
        match general_purpose::STANDARD.decode(found_keys.first().unwrap()) {
            Ok(bytes) => {
                match String::from_utf8(bytes) {
                    Ok(s) => found = s,
                    Err(e) => {
                        eprintln!("Error converting bytes to String: {}", e);
                    }
                };
            }
            Err(e) => {
                eprintln!("Error decoding Base64: {}", e);
            }
        };
        Some(found)
    }
}
