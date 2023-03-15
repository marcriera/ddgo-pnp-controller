use std::thread;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

use crate::state::ControllerState;
mod dgoc44u;
mod tcpp20009;
mod vok00106;

#[derive(PartialEq)]
pub enum ControllerModel {
    NONE,
    DGOC44U,
    TCPP20009,
    VOK00106,
}

pub fn set_model(state: &ControllerState) -> ControllerModel {
    if state.button_right {
        println!("Selected controller DGOC44-U.");
        init_gadget();
        return ControllerModel::DGOC44U;
    }
    else if state.button_up {
        println!("Selected controller TCPP-20009.");
        return ControllerModel::TCPP20009;
    }
    else if state.button_a {
        println!("Selected controller VOK-00106.");
        return ControllerModel::VOK00106;
    }
    else {
        println!("No controller selected.");
        return ControllerModel::NONE;
    }
}

pub fn set_state(state: &mut ControllerState, model: &ControllerModel) {
    match model {
        ControllerModel::DGOC44U => {
            dgoc44u::update_gadget(state);
        }
        ControllerModel::TCPP20009 => {
            tcpp20009::update_gadget(state);
        }
        ControllerModel::VOK00106 => {
            vok00106::update_gadget(state);
        }
        _ => (),
    }
}

fn init_gadget() {
    Command::new("modprobe").args(["g_ffs"]).output();
    Command::new("mkdir").args(["-p","/tmp/ffs-mascon"]).output();
    Command::new("mount").args(["-t","functionfs","mascon","/tmp/ffs-mascon"]).output();

    let descriptors = [0x03, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
                                0x09, 0x04, 0x00, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00,
                                0x07, 0x05, 0x81, 0x03, 0x08, 0x00, 0x00];
    let strings = [0x02, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    thread::spawn(move || {
        if let Ok(mut ep0) = File::open("/tmp/ffs-mascon/ep0") {
            let mut buffer = [0; 100];
            loop {
                ep0.read(&mut buffer).ok();
            }
        }
    });
    if let Ok(mut ep0) = File::create("/tmp/ffs-mascon/ep0") {
        ep0.write(&descriptors).ok();
        println!("USB Gadget: Descriptors written to EP0");
        ep0.write(&strings).ok();
        println!("USB Gadget: Strings written to EP0");
    }
}
