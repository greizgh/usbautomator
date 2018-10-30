use std;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use serde_derive::{Deserialize, Serialize};
use toml;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub devices: HashMap<String, WatchedDevice>,
}

#[derive(Deserialize, Serialize)]
pub struct WatchedDevice {
    pub on_plugged: String,
    pub on_unplugged: String,
    pub properties: HashMap<String, String>,
}

impl Config {
    pub fn from_file(path: std::path::PathBuf) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        match toml::from_str(&buffer) {
            Ok(config) => Ok(config),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        let mut properties = HashMap::new();
        // Ergodox sample values
        properties.insert("ID_VENDOR_ID".to_owned(), "feed".to_owned());
        properties.insert("ID_MODEL_ID".to_owned(), "1307".to_owned());
        properties.insert("ID_INPUT_KEYBOARD".to_owned(), "1".to_owned());
        properties.insert("KEY".to_owned(), "1000000000007 ff9f207ac14057ff febeffdfffefffff fffffffffffffffe".to_owned());
        let mut devices = HashMap::new();
        devices.insert(
            "ergodox".to_owned(),
            WatchedDevice {
                properties,
                on_plugged: "setxkbmap us".to_owned(),
                on_unplugged: "setxkbmap fr oss -option ctrl:nocaps".to_owned(),
            },
        );
        Config { devices }
    }
}
