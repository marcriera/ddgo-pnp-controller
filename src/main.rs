mod physical_controller;
mod virtual_controller;
mod state;

use std::io::Result;
use std::process::Command;
use std::time::Duration;
use std::thread::sleep;

use virtual_controller::ControllerModel;

fn main() -> Result<()> {
    match physical_controller::init() {
        Ok(mut dev) => {
            // Wait 3 seconds and get current state of the controller
            println!("Press a button to select the controller model...");
            sleep(Duration::from_secs(3));
            let mut controller_state = physical_controller::get_state(dev);
            let controller_model = virtual_controller::set_model(&controller_state);

            // If no model selected, quit
            if controller_model == ControllerModel::NONE {
                return Result::Ok(());
            }

            // Vibrate to end selection mode
            physical_controller::set_rumble(true);
            sleep(Duration::from_millis(500));
            physical_controller::set_rumble(false);

            // Stop main game
            stop_game();

            loop {
                // Process events from input devices
                /*for mut device in [&mut d1, &mut d2] {
                    let evs = device.fetch_events();
                    match evs {
                        Ok(evs) => {
                            for event in evs {
                                if event.event_type() == EventType::KEY {
                                    physical_controller::read_input(&mut controller_state, Key(event.code()), event.value());
                                }
                            }
                        },
                        Err(_e) => (),
                    }
                }*/
            }
        },
        Err(_e) => println!("ERROR: Could not read input devices! Exiting."),
    }
    Ok(())
}

fn stop_game() {
    Command::new("/etc/init.d/S99dgtype3").arg("stop").output().ok();
}