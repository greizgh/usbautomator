use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;
use std;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub devices: HashMap<String, WatchedDevice>,
}

#[derive(Deserialize)]
pub struct WatchedDevice {
    pub product: String,
    pub on_plugged: String,
    pub on_unplugged: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let config: Config = toml::from_str(&buffer).unwrap();

        Ok(config)
    }
}
