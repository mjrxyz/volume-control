// main.rs
extern crate evdev;

use evdev::{Device, EventType, ReadFlag};
use std::process::Command;

fn main() {
    // Specify the path to your input event device.
    // You can find your keyboard event device path using the `ls /dev/input/` command.
    let device_path = "/dev/input/eventX"; // Replace X with the appropriate number

    // Open the input event device
    let device = Device::open(&device_path).expect("Failed to open input device");

    // Listen for key events
    loop {
        // Read events from the device
        let events = device
            .events(ReadFlag::NORMAL)
            .expect("Failed to read input events");

        // Handle each event
        for event in events {
            match event.event_type {
                EventType::EV_KEY => {
                    match event.value {
                        1 => {
                            // Key press event
                            match event.code {
                                evdev::enums::EV_KEY::KEY_VOLUMEUP => {
                                    println!("Volume Up key pressed");
                                    increase_volume();
                                }
                                evdev::enums::EV_KEY::KEY_VOLUMEDOWN => {
                                    println!("Volume Down key pressed");
                                    decrease_volume();
                                }
                                _ => {}
                            }
                        }
                        0 => {
                            // Key release event
                            match event.code {
                                evdev::enums::EV_KEY::KEY_VOLUMEUP => {
                                    println!("Volume Up key released");
                                    stop_volume_action();
                                }
                                evdev::enums::EV_KEY::KEY_VOLUMEDOWN => {
                                    println!("Volume Down key released");
                                    stop_volume_action();
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

fn increase_volume() {
    // Increase the volume using PulseAudio pactl command
    Command::new("pactl")
        .args(&["set-sink-volume", "@DEFAULT_SINK@", "+5%"])
        .spawn()
        .expect("Failed to execute pactl command to increase volume");
}

fn decrease_volume() {
    // Decrease the volume using PulseAudio pactl command
    Command::new("pactl")
        .args(&["set-sink-volume", "@DEFAULT_SINK@", "-5%"])
        .spawn()
        .expect("Failed to execute pactl command to decrease volume");
}

fn stop_volume_action() {
    // Placeholder for your key release logic
    println!("Stop volume action");
// Add the volume release logic.
}
