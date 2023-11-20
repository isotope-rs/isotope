use std::error::Error;

use crate::config::{save_config, Conf};
use base64::{engine::general_purpose, Engine as _};

impl Conf {
    fn encode_cache_key(&self, raw_cache_key: &str) -> String {
        general_purpose::STANDARD.encode(raw_cache_key)
    }

    pub fn upsert_into_cache(mut self, raw_cache_key: &str, payload: &str) -> Self {
        let encoded_key = self.encode_cache_key(raw_cache_key);
        let encoded_payload = self.encode_cache_key(payload);
        // append the new data to the hashmap
        self.stored_advice.insert(encoded_key, encoded_payload);

        // In this scenario we cannot use ? operand as it impacts creation of the struct
        match save_config(&self) {
            Ok(_x) => self,
            Err(_e) => panic!(),
        }
    }
    pub fn remove_from_cache(mut self, raw_cache_key: &str) -> Result<Self, Box<dyn Error>> {
        let encoded_key = self.encode_cache_key(raw_cache_key);
        self.stored_advice.remove(encoded_key.as_str());
        save_config(&self)?;
        Ok(self)
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
