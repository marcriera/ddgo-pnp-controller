use std::fs::File;
use std::io::Write;

use bitflags::bitflags;
use crate::controller::physical::ControllerState;
use crate::controller::emulated::{DeviceDescriptor, ENDPOINT1};

pub const DESCRIPTORS: [u8; 48] = [0x01, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
0x09, 0x04, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
0x07, 0x05, 0x81, 0x03, 0x08, 0x00, 0x14,
0x09, 0x04, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
0x07, 0x05, 0x81, 0x03, 0x08, 0x00, 0x14];
pub const STRINGS: [u8; 16] = [0x02, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

pub const DEVICE_DESCRIPTOR: DeviceDescriptor = DeviceDescriptor{b_device_class: 0x00, b_device_sub_class: 0x0, id_vendor: 0x1C06, id_product: 0x77A7, bcd_device: 0x0102, i_manufacturer: "TAITO", i_product: "Densha de Go! Plug & Play (MTC P5/B5 mode)", i_serial_number: "SOTP-031201"};

const POWER_NOTCHES: [u8; 6] = [0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C];
const BRAKE_NOTCHES: [u8; 10] = [0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x02, 0x02, 0x02, 0x01];

bitflags! {
    struct Buttons1: u8 {
        const NONE = 0;
        const S = 1;
        const D = 2;
        const A = 4;
        const A2 = 8;
        const B = 16;
        const C = 32;
    }
    struct Buttons2: u8 {
        const NONE = 0;
        const START = 1;
        const SELECT = 2;
        const UP = 4;
        const DOWN = 8;
        const LEFT = 16;
        const RIGHT = 32;
    }
}

pub fn update_gadget(state: &mut ControllerState) {
    // Calculate data for handles
    let mut handle = POWER_NOTCHES[state.power as usize];
    if state.brake > 0 {
        handle = BRAKE_NOTCHES[state.brake as usize];
    }

    // Calculate data for buttons
    let mut buttons1 = Buttons1::NONE;
    let mut buttons2 = Buttons2::NONE;
    if !state.button_select_hold && state.button_select && state.button_a {
        buttons1.insert(Buttons1::A2);
        state.combo = true;
    }
    if !state.button_select_hold && state.button_select && state.button_d {
        buttons1.insert(Buttons1::S);
        state.combo = true;
    }
    if !state.button_select_hold && state.button_select && state.button_up {
        state.reverser = 0x20;
        state.combo = true;
    }
    if !state.button_select_hold && state.button_select && state.button_left {
        state.reverser = 0x00;
        state.combo = true;
    }
    if !state.button_select_hold && state.button_select && state.button_down {
        state.reverser = 0x10;
        state.combo = true;
    }
    if !state.combo && state.button_a { buttons1.insert(Buttons1::A) }
    if state.button_b { buttons1.insert(Buttons1::B) }
    if !state.combo && state.button_c { buttons1.insert(Buttons1::C) }
    if state.button_d { buttons1.insert(Buttons1::D) }
    if !state.combo && state.button_up { buttons2.insert(Buttons2::UP) }
    if !state.combo && state.button_down { buttons2.insert(Buttons2::DOWN) }
    if !state.combo && state.button_left { buttons2.insert(Buttons2::LEFT) }
    if state.button_right { buttons2.insert(Buttons2::RIGHT) }
    if state.button_start { buttons2.insert(Buttons2::START) }
    if !state.combo && state.button_select_hold { buttons2.insert(Buttons2::SELECT) }

    // Assemble data and send it to endpoint
    let data = [0x1, state.reverser + handle, buttons1.bits, buttons2.bits];
    if let Ok(mut file) = File::create(ENDPOINT1) {
        file.write(&data).ok();
    }
}