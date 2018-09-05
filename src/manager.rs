use config;
use libudev::{Device, Event, EventType};
use notify_rust::Notification;
use std::process::Command;
use std::collections::HashMap;

pub struct DeviceManager {
    pub config: config::Config,
}

impl DeviceManager {
    pub fn handle_device(&self, device: &Device) {
        for (name, watched) in &self.config.devices {
            if is_matching(&watched.properties, device) {
                notify(&format!("{} plugged", name));
                execute(&watched.on_plugged);
            }
        }
    }

    pub fn handle_change(&self, event: &Event) {
        for (name, watched) in &self.config.devices {
            if is_matching(&watched.properties, event.device()) {
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

/// Check if device match with given property set
fn is_matching(properties: &HashMap<String, String>, device: &Device) -> bool {
    if device.subsystem() != "input" || device.subsystem() != "block" {
        return false;
    }
    let mut matching_properties: usize = 0;
    for property in device.properties() {
        let name = property.name().to_os_string().into_string().expect("Could not convert property name");
        let value = property.value().to_os_string().into_string().expect("Could not convert property value");

        if properties.contains_key(&name) && properties.get(&name).unwrap() == &value {
            matching_properties += 1;
        }
    }

    // Prevent matching on empty property set
    !properties.is_empty() && matching_properties == properties.len()
}

fn execute(command: &str) {
    let cmd: Vec<_> = command.split(' ').collect();
    let run = Command::new(cmd[0])
        .args(&cmd[1..])
        .spawn();
    if run.is_ok() {
        run.unwrap();
    } else {
        eprintln!("Failed to run command: {}", command);
    }
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
