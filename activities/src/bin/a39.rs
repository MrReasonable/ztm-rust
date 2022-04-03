// Topic: Channels
//
// Summary:
//   Using the existing code, create a program that simulates an internet-of-things
//   remote control light bulb. The color of the light can be changed remotely.
//   Use threads and channels to communicate what color the light bulb should display.
//
// Requirements:
// * Create a separate thread representing the light bulb
// * Use a channel to communicate with the thread
// * Display a color change message using the println! macro
// * The light bulb must also be able to turn on and off
//   * Display whether the light is on or off on each color change
// * Turn off the light when disconnecting from it
//
// Notes:
// * Remember to add `crossbeam-channel` to your Cargo.toml file
// * Use the `colored` crate if you want to get fancy and display actual colors
// * The docs.rs site can be used to read documentation for third-party crates
// * Disconnection can be accomplished by dropping the sender, or
//   by telling the thread to self-terminate
// * Use `cargo test --bin a39` to test your program to ensure all cases are covered

use colored::Colorize;
use crossbeam_channel::{unbounded, Receiver};
use std::thread::{self, JoinHandle};

enum LightMsg {
    // Add additional variants needed to complete the exercise
    ChangeColor(u8, u8, u8),
    Disconnect,
    On,
    Off,
}

enum LightStatus {
    Off,
    On,
}

fn spawn_light_thread(receiver: Receiver<LightMsg>) -> JoinHandle<LightStatus> {
    thread::spawn(move || {
        use LightMsg::*;
        let mut light_status = LightStatus::Off;
        loop {
            if let Ok(msg) = receiver.recv() {
                match msg {
                    On => {
                        println!("Light turned ON");
                        light_status = LightStatus::On
                    }
                    Off => {
                        println!("Light turned OFF");
                        light_status = LightStatus::Off
                    }
                    ChangeColor(r, g, b) => {
                        println!("Color changed to {}", "   ".on_truecolor(r, g, b));
                        match light_status {
                            LightStatus::Off => println!("Light is OFF"),
                            LightStatus::On => println!("Light is ON"),
                        }
                    }
                    Disconnect => {
                        println!("Disconnecting");
                        light_status = LightStatus::Off;
                        break;
                    }
                }
            } else {
                println!("Channel disconnected");
                light_status = LightStatus::Off;
                break;
            }
        }
        light_status
    })
}

fn main() {
    let (tx, rx) = unbounded();
    let light = spawn_light_thread(rx);
    tx.send(LightMsg::On).expect("Unable to switch light on");
    tx.send(LightMsg::ChangeColor(255, 255, 255))
        .expect("Unable to change color");
    tx.send(LightMsg::ChangeColor(50, 30, 18))
        .expect("Unable to change color");
    tx.send(LightMsg::ChangeColor(95, 32, 255))
        .expect("Unable to change color");
    tx.send(LightMsg::Off).expect("Unable to switch light off");
    tx.send(LightMsg::Disconnect).expect("Unable to disconnect");
    let _status = light.join().expect("Unable to join light");
}

#[cfg(test)]
mod test {
    use super::*;
    use crossbeam_channel::unbounded;

    #[test]
    fn light_off_when_disconnect() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        s.send(LightMsg::Disconnect).expect("channel disconnected");

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after disconnection");
        }
    }

    #[test]
    fn light_off_when_dropped() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        drop(s);

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after dropping sender");
        }
    }
}
