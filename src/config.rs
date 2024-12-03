use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub inactivity_timeout: u64,
    pub auto_connect: bool,
    pub device_address: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string("config.json")?;
        let config: Config = serde_json::from_str(&config_str)?;
        Ok(config)
    }

    #[allow(dead_code)]
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write("config.json", config_str)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            inactivity_timeout: 300,
            auto_connect: true,
            device_address: String::from("XX:XX:XX:XX:XX:XX"),
        }
    }
}
