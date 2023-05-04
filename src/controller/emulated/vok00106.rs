use std::fs::File;
use std::io::{Write};
use crate::controller::physical::ControllerState;
use crate::controller::emulated::{DeviceDescriptor, ENDPOINT1};

pub const DESCRIPTORS: [u8; 76] = [0x01, 0x00, 0x00, 0x00, 0x4C, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
0x09, 0x04, 0x00, 0x00, 0x03, 0xFF, 0x00, 0x00, 0x00,
0x07, 0x05, 0x82, 0x02, 0x20, 0x00, 0x00,
0x07, 0x05, 0x02, 0x02, 0x20, 0x00, 0x00,
0x07, 0x05, 0x81, 0x03, 0x08, 0x00, 0x01,
0x09, 0x04, 0x00, 0x00, 0x03, 0xFF, 0x00, 0x00, 0x00,
0x07, 0x05, 0x82, 0x02, 0x20, 0x00, 0x00,
0x07, 0x05, 0x02, 0x02, 0x20, 0x00, 0x00,
0x07, 0x05, 0x81, 0x03, 0x08, 0x00, 0x01];
pub const STRINGS: [u8; 16] = [0x02, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

pub const DEVICE_DESCRIPTOR: DeviceDescriptor = DeviceDescriptor{b_device_class: 0x02, b_device_sub_class: 0x0, id_vendor: 0x067B, id_product: 0x2303, bcd_device: 0x0102, i_manufacturer: "TAITO", i_product: "Densha de Go! Plug & Play (Master Controller II mode)", i_serial_number: "VOK-00106"};

const POWER_NOTCHES: [&str; 6] = ["TSA50", "TSA55", "TSA65", "TSA75", "TSA85", "TSA95"];
const BRAKE_NOTCHES: [&str; 10] = ["TSA50", "TSA45", "TSA35", "TSA25", "TSA15", "TSA05", "TSE99", "TSB40", "TSB30", "TSB20"];
const BUTTON_A: [&str; 2] = ["TSX00", "TSX99"];
const BUTTON_B: [&str; 2] = ["TSY00", "TSY99"];
const BUTTON_C: [&str; 2] = ["TSZ00", "TSZ99"];
const BUTTON_S: [&str; 2] = ["TSK00", "TSK99"];


pub fn update_gadget(state: &mut ControllerState) {
    // Calculate data for handles
    let mut handle = POWER_NOTCHES[state.power as usize];
    if state.brake > 0 {
        handle = BRAKE_NOTCHES[state.brake as usize];
    }

    // Calculate data for buttons
    let button_a = if state.button_a {BUTTON_A[1]} else {BUTTON_A[0]};
    let button_b = if state.button_b {BUTTON_B[1]} else {BUTTON_B[0]};
    let button_c = if state.button_c {BUTTON_C[1]} else {BUTTON_C[0]};
    let button_s = if state.button_d {BUTTON_S[1]} else {BUTTON_S[0]};

    if let Ok(mut file) = File::create(ENDPOINT1) {
        file.write(handle.as_bytes()).ok();
        file.write(&[0xD]).ok();
        file.write(button_a.as_bytes()).ok();
        file.write(&[0xD]).ok();
        file.write(button_b.as_bytes()).ok();
        file.write(&[0xD]).ok();
        file.write(button_c.as_bytes()).ok();
        file.write(&[0xD]).ok();
        file.write(button_s.as_bytes()).ok();
        file.write(&[0xD]).ok();
    }
}