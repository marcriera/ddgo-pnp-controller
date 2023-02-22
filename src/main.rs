use std::fs::File;
use std::io::Write;
use std::io::Result;
use std::process::Command;
use std::time::{Duration, Instant};
use std::thread::sleep;

use evdev_rs::Device;
use evdev_rs::InputEvent;
use evdev_rs::ReadFlag;
use evdev_rs::enums::EventCode;
use evdev_rs::enums::EV_KEY;

#[derive(Default)]
struct ControllerState {
    power: u8,
    brake: u8,
    button_sl: bool,
    button_st: bool,
    button_a: bool,
    button_b: bool,
    button_c: bool,
    button_d: bool,
    button_up: bool,
    button_down: bool,
    button_left: bool,
    button_right: bool,
}

#[derive(PartialEq)]
enum ControllerModel {
    NONE,
    DGOC44U,
    TYPE2,
}

fn main() -> Result<()> {
    let d1 = Device::new_from_path("/dev/input/event1");
    let d2 = Device::new_from_path("/dev/input/event2");

    match (d1, d2) {
        (Ok(d1), Ok(d2)) => {
            let mut controller_state: ControllerState = Default::default();
            let mut controller_model: ControllerModel = ControllerModel::NONE;
        
            // Save current time and 5 seconds in the future to check for pressed buttons later
            let start_time = Instant::now();
            let init_time = start_time + Duration::from_secs(5);
        
            // Turn on door light to indicate selection mode 
            set_lamp(true);
            println!("Hold a button to select the controller model...");

            loop {
                // Process events from both input devices
                for device in [&d1, &d2] {
                    let ev = device.next_event(ReadFlag::NORMAL);
                    match ev {
                        Ok(ev) => read_input(ev.1, &mut controller_state),
                        Err(_e) => (),
                    }
                }
        
                // If init time has passed, try to select model or quit
                if controller_model == ControllerModel::NONE && Instant::now() >= init_time {
                    if controller_state.button_right {
                        controller_model = ControllerModel::DGOC44U;
                        println!("Selected controller DGOC44-U, starting gadget...");
                    }
                    else if controller_state.button_left {
                        controller_model = ControllerModel::TYPE2;
                        println!("Selected controller TCPP-20009, starting gadget...");
                    }
                    else
                    {
                        // Turn off door light and quit
                        set_lamp(false);
                        println!("No controller selected, exiting...");
                        break;
                    }
                    // Turn off door light and vibrate to end selection mode
                    set_lamp(false);
                    set_rumble(true);
                    sleep(Duration::from_millis(500));
                    set_rumble(false);

                    // Stop main game
                    stop_game();
                }
            }
        },
        _ => println!("ERROR: Could not read input devices! Stopping..."),
    }
    Ok(())
}

fn read_input(event: InputEvent, controller: &mut ControllerState) {
    // Save input status to object for easier processing
    match event.event_code{
        EventCode::EV_KEY(EV_KEY::KEY_0)=>if event.value == 1 {controller.power = 0},
        EventCode::EV_KEY(EV_KEY::KEY_1)=>if event.value == 1 {controller.power = 1},
        EventCode::EV_KEY(EV_KEY::KEY_2)=>if event.value == 1 {controller.power = 2},
        EventCode::EV_KEY(EV_KEY::KEY_3)=>if event.value == 1 {controller.power = 3},
        EventCode::EV_KEY(EV_KEY::KEY_4)=>if event.value == 1 {controller.power = 4},
        EventCode::EV_KEY(EV_KEY::KEY_5)=>if event.value == 1 {controller.power = 5},
        EventCode::EV_KEY(EV_KEY::KEY_B)=>if event.value == 1 {controller.brake = 0},
        EventCode::EV_KEY(EV_KEY::KEY_C)=>if event.value == 1 {controller.brake = 1},
        EventCode::EV_KEY(EV_KEY::KEY_D)=>if event.value == 1 {controller.brake = 2},
        EventCode::EV_KEY(EV_KEY::KEY_E)=>if event.value == 1 {controller.brake = 3},
        EventCode::EV_KEY(EV_KEY::KEY_F)=>if event.value == 1 {controller.brake = 4},
        EventCode::EV_KEY(EV_KEY::KEY_G)=>if event.value == 1 {controller.brake = 5},
        EventCode::EV_KEY(EV_KEY::KEY_H)=>if event.value == 1 {controller.brake = 6},
        EventCode::EV_KEY(EV_KEY::KEY_I)=>if event.value == 1 {controller.brake = 7},
        EventCode::EV_KEY(EV_KEY::KEY_J)=>if event.value == 1 {controller.brake = 8},
        EventCode::EV_KEY(EV_KEY::KEY_P)=>if event.value == 1 {controller.brake = 9},
        EventCode::EV_KEY(EV_KEY::KEY_SPACE)=>controller.button_sl = event.value != 0,        
        EventCode::EV_KEY(EV_KEY::KEY_ENTER)=>controller.button_st = event.value != 0,        
        EventCode::EV_KEY(EV_KEY::KEY_A)=>controller.button_a = event.value != 0,        
        EventCode::EV_KEY(EV_KEY::KEY_Z)=>controller.button_b = event.value != 0,        
        EventCode::EV_KEY(EV_KEY::KEY_X)=>controller.button_c = event.value != 0,        
        EventCode::EV_KEY(EV_KEY::KEY_S)=>controller.button_d = event.value != 0,  
        EventCode::EV_KEY(EV_KEY::KEY_UP)=>controller.button_up = event.value != 0,        
        EventCode::EV_KEY(EV_KEY::KEY_DOWN)=>controller.button_down = event.value != 0,        
        EventCode::EV_KEY(EV_KEY::KEY_LEFT)=>controller.button_left = event.value != 0,        
        EventCode::EV_KEY(EV_KEY::KEY_RIGHT)=>controller.button_right = event.value != 0,        
        _=>(),
    }
}

fn set_lamp(status: bool) {
    if let Ok(mut out) = File::create("/sys/class/leds/led2/brightness") {
        out.write(if status {b"1"} else {b"0"}).ok(); 
    }
    else {
        println!("WARNING: Could not set door lamp status!")
    }
}

fn set_rumble(status: bool) {
    if let Ok(mut out) = File::create("/sys/class/leds/led1/brightness") {
        out.write(if status {b"1"} else {b"0"}).ok(); 
    }
    else {
        println!("WARNING: Could not set rumble motor status!")
    }
}

fn stop_game() {
    Command::new("/etc/init.d/S99dgtype3").arg("stop").output().ok();
}