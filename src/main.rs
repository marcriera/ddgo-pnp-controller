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
        Ok(dev) => {
            // Wait 3 seconds and get current state of the controller
            println!("Press a button to select the controller model...");
            sleep(Duration::from_secs(3));
            let mut controller_state = Default::default();
            physical_controller::get_state(&mut controller_state, &dev);
            let controller_model = virtual_controller::set_model(&controller_state);

            // If no model selected, quit
            if controller_model == ControllerModel::NONE {
                return Result::Ok(());
            }

            // Stop main game
            stop_game();

            // Vibrate to end selection mode
            physical_controller::set_rumble(true);
            sleep(Duration::from_millis(500));
            physical_controller::set_rumble(false);

            loop {
                // Fetch events from input devices
                physical_controller::get_state(&mut controller_state, &dev);
                sleep(Duration::from_millis(100));
            }
        },
        Err(_e) => println!("ERROR: Could not read input devices! Exiting."),
    }
    Ok(())
}

fn stop_game() {
    Command::new("/etc/init.d/S99dgtype3").arg("stop").output().ok();
}