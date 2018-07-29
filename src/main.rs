extern crate libudev;
extern crate notify_rust;

use libudev::{Context, Device, Enumerator, Monitor, EventType};
use notify_rust::Notification;

/// USB Vendor/Product as defined in QMK firmware
const PRODUCT: &str = "3/feed/1307/111";

fn main() {
    let context = Context::new().unwrap();
    let mut enumerator = Enumerator::new(&context).unwrap();

    enumerator.match_subsystem("input").unwrap();

    for device in enumerator.scan_devices().unwrap() {
        if is_keyboard(&device) {
            on_keyboard_plugged();
            break;
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
                        EventType::Add => on_keyboard_plugged(),
                        EventType::Remove => on_keyboard_unplugged(),
                        _ => {},
                    }
                }
            }
            None => {},
        }
    }
}

fn on_keyboard_plugged() {
    Notification::new()
        .summary("Auto xkbmap")
        .body("Keyboard set")
        .icon("keyboard")
        .show().unwrap();
}

fn on_keyboard_unplugged() {
    Notification::new()
        .summary("Auto xkbmap")
        .body("Keyboard unset")
        .icon("keyboard")
        .show().unwrap();
}

fn is_keyboard(device: &Device) -> bool {
    for property in device.properties() {
        if property.name() == "PRODUCT" && property.value() == PRODUCT {
            return true;
        }
    }

    false
}
