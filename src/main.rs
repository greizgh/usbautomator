extern crate libudev;

use libudev::{Context, Device, Enumerator, Monitor, EventType};

/// USB Vendor/Product as defined in QMK firmware
const PRODUCT: &str = "3/feed/1307/111";

fn main() {
    let context = Context::new().unwrap();
    let mut enumerator = Enumerator::new(&context).unwrap();

    enumerator.match_subsystem("input").unwrap();

    for device in enumerator.scan_devices().unwrap() {
        if is_keyboard(&device) {
            println!("keyboard detected");
        }
    }

    let mut monitor = Monitor::new(&context).unwrap();
    assert!(monitor.match_subsystem("input").is_ok());
    let mut socket = monitor.listen().unwrap();

    loop {
        match socket.receive_event() {
            Some(event) => {
                if is_keyboard(event.device()) {
                    match event.event_type() {
                        EventType::Add => println!("keyboard plugged"),
                        EventType::Remove => println!("keyboard unplugged"),
                        _ => {},
                    }
                }
            }
            None => {},
        }
    }
}

fn is_keyboard(device: &Device) -> bool {
    for property in device.properties() {
        if property.name() == "PRODUCT" && property.value() == PRODUCT {
            return true;
        }
    }

    false
}
