// main.rs
extern crate evdev;

use evdev::{Device, EventType, ReadFlag};
use std::process::Command;

fn main() {
    // Specify the path to your input event device.
    // You can find your keyboard event device path using the `ls /dev/input/` command.
    let device_path = "/dev/input/eventX"; // Replace X with the appropriate number


    let device = Device::open(&device_path).expect("Failed to open input device");


    loop {

        let events = device
            .events(ReadFlag::NORMAL)
            .expect("Failed to read input events");
        
        for event in events {
            match event.event_type {
                EventType::EV_KEY => {
                    match event.value {
                        1 => {

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

    Command::new("pactl")
        .args(&["set-sink-volume", "@DEFAULT_SINK@", "+5%"])
        .spawn()
        .expect("Failed to execute pactl command to increase volume");
}

fn decrease_volume() {

    Command::new("pactl")
        .args(&["set-sink-volume", "@DEFAULT_SINK@", "-5%"])
        .spawn()
        .expect("Failed to execute pactl command to decrease volume");
}

fn stop_volume_action() {
    // Placeholder for key release logic
    println!("Stop volume action");

}
