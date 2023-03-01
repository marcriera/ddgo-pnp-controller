mod controller;
mod state;

use std::io::Result;
use std::process::Command;
use std::time::Duration;
use std::thread::sleep;

use controller::emulated::ControllerModel;
use controller::physical::{set_lamp,set_rumble};

fn main() -> Result<()> {
    match controller::physical::init() {
        Ok(dev) => {
            // Wait 3 seconds and get current state of the controller
            println!("Press a button to select the controller model...");
            sleep(Duration::from_secs(3));
            let mut controller_state = Default::default();
            controller::physical::get_state(&mut controller_state, &dev);
            let controller_model = controller::emulated::set_model(&controller_state);

            // If no model selected, quit
            if controller_model == ControllerModel::NONE {
                return Result::Ok(());
            }

            // Stop main game
            stop_game();

            // Vibrate to end selection mode
            set_rumble(true);
            sleep(Duration::from_millis(500));
            set_rumble(false);

            loop {
                // Fetch events from input devices
                controller::physical::get_state(&mut controller_state, &dev);

                // Send input to virtual controller
                controller::emulated::set_state(&mut controller_state, &controller_model);

                // Update lamp and rumble
                set_lamp(controller_state.lamp);
                set_rumble(controller_state.rumble);

                // Wait between cycles
                sleep(Duration::from_millis(20));
            }
        },
        Err(_e) => println!("ERROR: Could not read input devices! Exiting."),
    }
    Ok(())
}

fn stop_game() {
    Command::new("/etc/init.d/S99dgtype3").arg("stop").output().ok();
}