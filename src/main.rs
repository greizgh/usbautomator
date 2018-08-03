extern crate libudev;
extern crate notify_rust;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate xdg;

use libudev::{Context, Enumerator, Monitor};
use std::fs::File;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

mod config;
mod manager;

const SLEEP_DURATION: u64 = 1000;

fn main() {
    let config = get_config().unwrap();
    let manager = manager::DeviceManager { config };

    let context = Context::new().unwrap();
    let mut enumerator = Enumerator::new(&context).unwrap();

    enumerator.match_subsystem("input").unwrap();

    for device in enumerator.scan_devices().unwrap() {
        manager.handle_device(&device);
    }

    let mut monitor = Monitor::new(&context).unwrap();
    assert!(monitor.match_subsystem("input").is_ok());
    let mut socket = monitor.listen().unwrap();

    loop {
        match socket.receive_event() {
            Some(event) => manager.handle_change(&event),
            None => sleep(Duration::from_millis(SLEEP_DURATION)),
        }
    }
}

/// Load configuration from file.
/// Create configuration file from defaults if needed.
fn get_config() -> Result<config::Config, std::io::Error> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"))?;
    let config_path = xdg_dirs.place_config_file("config.toml")?;

    if !config_path.exists() {
        let toml = toml::to_string(&config::Config::default()).unwrap();
        let mut file = File::create(&config_path)?;
        file.write_all(toml.as_bytes())?;
    }

    config::Config::from_file(config_path)
}
