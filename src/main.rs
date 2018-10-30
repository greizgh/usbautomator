use libudev::{Context, Enumerator, Monitor};
use std::fs::File;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;

mod config;
mod manager;

const SLEEP_DURATION: u64 = 1000;

#[derive(Debug, StructOpt)]
struct Opt {
    /// List connected devices of type input or block
    #[structopt(short = "l", long = "list")]
    list: bool,
    /// Watch for changes and display changed device properties
    #[structopt(short = "w", long = "watch")]
    watch: bool,
    /// List properties of device identified by given name
    #[structopt(long = "describe")]
    device_name: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    let config = get_config().unwrap();
    let manager = manager::DeviceManager { config };

    let context = Context::new().unwrap();

    match opt {
        Opt { list: true, .. } => {
            list_devices(&context);
            return;
        },
        Opt { watch: true, .. } => {
            watch(&context);
            return;
        }
        Opt { device_name: Some(name), .. } => {
            describe(&context, &name);
            return;
        },
        _ => listen(&manager, &context),
    }
}

/// List input or block devices
fn list_devices(context: &Context) {
    let mut input_enumerator = Enumerator::new(context).unwrap();
    input_enumerator.match_subsystem("input").unwrap();
    for device in input_enumerator.scan_devices().unwrap() {
        println!("Device input\t\t{}", device.sysname().to_string_lossy());
    }

    let mut block_enumerator = Enumerator::new(context).unwrap();
    block_enumerator.match_subsystem("block").unwrap();
    for device in block_enumerator.scan_devices().unwrap() {
        println!("Device block\t\t{}", device.sysname().to_string_lossy());
    }
}

/// Describe device by listing its properties
fn describe(context: &Context, name: &str) {
    let mut enumerator = Enumerator::new(context).unwrap();

    enumerator.match_sysname(name).unwrap();
    for device in enumerator.scan_devices().unwrap() {
        println!("******************************************************************************");
        for property in device.properties() {
            println!("{}: {}", property.name().to_string_lossy(), property.value().to_string_lossy());
        }
    }
}

/// Listen for changes on plugged devices
fn listen(manager: &manager::DeviceManager, context: &Context) {
    let mut enumerator = Enumerator::new(context).unwrap();

    for device in enumerator.scan_devices().unwrap() {
        manager.handle_device(&device);
    }

    let monitor = Monitor::new(context).unwrap();
    let mut socket = monitor.listen().unwrap();

    loop {
        match socket.receive_event() {
            Some(event) => manager.handle_change(&event),
            None => sleep(Duration::from_millis(SLEEP_DURATION)),
        }
    }
}

/// Watch for changes and display the device properties
fn watch(context: &Context) {
    let monitor = Monitor::new(context).unwrap();
    let mut socket = monitor.listen().unwrap();

    loop {
        match socket.receive_event() {
            Some(event) => describe(context, &event.device().sysname().to_string_lossy()),
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
