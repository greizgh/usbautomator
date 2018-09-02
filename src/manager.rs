use config;
use libudev::{Device, Event, EventType};
use notify_rust::Notification;
use std::process::Command;

pub struct DeviceManager {
    pub config: config::Config,
}

impl DeviceManager {
    pub fn handle_device(&self, device: &Device) {
        for (name, watched) in &self.config.devices {
            if is_product(&watched.product, device) {
                notify(&format!("{} plugged", name));
                execute(&watched.on_plugged);
            }
        }
    }

    pub fn handle_change(&self, event: &Event) {
        for (name, watched) in &self.config.devices {
            if is_product(&watched.product, event.device()) {
                match event.event_type() {
                    EventType::Add => {
                        notify(&format!("{} plugged", name));
                        execute(&watched.on_plugged);
                    }
                    EventType::Remove => {
                        notify(&format!("{} unplugged", name));
                        execute(&watched.on_unplugged);
                    }
                    _ => break,
                }
            }
        }
    }
}

fn is_product(product: &str, device: &Device) -> bool {
    for property in device.properties() {
        if property.name() == "PRODUCT" && property.value() == product {
            return true;
        }
    }

    false
}

fn execute(command: &str) {
    let cmd: Vec<_> = command.split(" ").collect();
    Command::new(cmd[0])
        .args(&cmd[1..])
        .spawn()
        .expect("Failed to run command");
}

fn notify(message: &str) {
    let notification = Notification::new()
        .summary("Auto xkbmap")
        .body(message)
        .icon("keyboard")
        .show();
    if notification.is_ok() {
        notification.unwrap();
    } else {
        eprintln!("Unable to notify: {}", message);
    }
}
