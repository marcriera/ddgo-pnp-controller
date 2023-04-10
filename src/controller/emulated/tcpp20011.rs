use std::fs::File;
use std::io::Write;

use bitflags::bitflags;
use crate::controller::physical::ControllerState;
use crate::controller::emulated::{DeviceDescriptor, ENDPOINT1};

pub const DESCRIPTORS: [u8; 48] = [0x01, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
0x09, 0x04, 0x00, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00,
0x07, 0x05, 0x81, 0x03, 0x08, 0x00, 0x14,
0x09, 0x04, 0x00, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00,
0x07, 0x05, 0x81, 0x03, 0x08, 0x00, 0x14];
pub const STRINGS: [u8; 16] = [0x02, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

pub const DEVICE_DESCRIPTOR: DeviceDescriptor = DeviceDescriptor{b_device_class: 0xFF, b_device_sub_class: 0x5, id_vendor: 0x0AE4, id_product: 0x0005, i_manufacturer: "TAITO", i_product: "Densha de Go! Plug & Play (Shinkansen)", i_serial_number: "TCPP20011"};

const POWER_NOTCHES: [u8; 6] = [0x12, 0x36, 0x5A, 0x90, 0xC6, 0xFB];
const BRAKE_NOTCHES: [u8; 10] = [0x1C, 0x38, 0x54, 0x70, 0x8B, 0xA7, 0xC3, 0xDF, 0xDF, 0xFB];

bitflags! {
        struct Buttons: u8 {
            const NONE = 0;
            const D = 1;
            const C = 2;
            const B = 4;
            const A = 8;
            const SELECT = 16;
            const START = 32;
        }
    }

pub fn update_gadget(state: &mut ControllerState) {
    // Calculate data for handles
    let power = POWER_NOTCHES[state.power as usize];
    let brake = BRAKE_NOTCHES[state.brake as usize];

    // Calculate data for buttons
    let mut buttons = Buttons::NONE;
    if state.button_a { buttons.insert(Buttons::A) }
    if state.button_b { buttons.insert(Buttons::B) }
    if state.button_c { buttons.insert(Buttons::C) }
    if state.button_d { buttons.insert(Buttons::D) }
    if state.button_select { buttons.insert(Buttons::SELECT) }
    if state.button_start { buttons.insert(Buttons::START) }

    // Calculate data for D-pad
    let mut dpad: u8 = 0x8;
    if state.button_up { dpad = 0x0 }
    if state.button_down { dpad = 0x4 }
    if state.button_left { dpad = 0x6 }
    if state.button_right { dpad = 0x2 }
    if state.button_up & state.button_left { dpad = 0x7 }
    if state.button_up & state.button_right { dpad = 0x1 }
    if state.button_down & state.button_left { dpad = 0x5 }
    if state.button_down & state.button_right { dpad = 0x3 }

    // Assemble data and send it to endpoint
    let data = [brake, power, 0xFF, dpad, buttons.bits, 0x0];
    if let Ok(mut file) = File::create(ENDPOINT1) {
        file.write(&data).ok();
    }
}