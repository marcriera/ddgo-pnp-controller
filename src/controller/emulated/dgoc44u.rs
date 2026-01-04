use std::fs::File;
use std::io::Write;

use crate::controller::emulated::{DeviceDescriptor, ENDPOINT1};
use crate::controller::physical::ControllerState;
use bitflags::bitflags;

pub const DESCRIPTORS: [u8; 66] = [
    0x01, 0x00, 0x00, 0x00, 0x42, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
    0x09, 0x04, 0x00, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00, 0x09, 0x21, 0x11, 0x01, 0x00, 0x01, 0x22,
    0x3F, 0x00, 0x07, 0x05, 0x81, 0x03, 0x08, 0x00, 0x05, 0x09, 0x04, 0x00, 0x00, 0x01, 0x03, 0x00,
    0x00, 0x00, 0x09, 0x21, 0x11, 0x01, 0x00, 0x01, 0x22, 0x3F, 0x00, 0x07, 0x05, 0x81, 0x03, 0x08,
    0x00, 0x05,
];
pub const STRINGS: [u8; 16] = [
    0x02, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub const DEVICE_DESCRIPTOR: DeviceDescriptor = DeviceDescriptor {
    b_device_class: 0x0,
    b_device_sub_class: 0x0,
    id_vendor: 0x0AE4,
    id_product: 0x0003,
    bcd_device: 0x0102,
    i_manufacturer: "TAITO",
    i_product: "電車でGO! コントローラ USB版",
    i_serial_number: "DGOC-44U_PNP",
};

pub const HID_REPORT_DESCRIPTOR: [u8; 63] = [
    0x05, 0x01, // Usage Page (Generic Desktop Ctrls)
    0x09, 0x04, // Usage (Joystick)
    0xA1, 0x01, // Collection (Application)
    0x09, 0x01, //   Usage (Pointer)
    0xA1, 0x00, //   Collection (Physical)
    0x09, 0x30, //     Usage (X)
    0x09, 0x31, //     Usage (Y)
    0x15, 0x00, //     Logical Minimum (0)
    0x26, 0xFF, 0x00, //     Logical Maximum (255)
    0x75, 0x08, //     Report Size (8)
    0x95, 0x02, //     Report Count (2)
    0x81, 0x02, //     Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0xC0, //   End Collection
    0x75, 0x08, //   Report Size (8)
    0x95, 0x01, //   Report Count (1)
    0x81, 0x01, //   Input (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x05, 0x09, //   Usage Page (Button)
    0x19, 0x01, //   Usage Minimum (0x01)
    0x29, 0x06, //   Usage Maximum (0x06)
    0x15, 0x00, //   Logical Minimum (0)
    0x25, 0x01, //   Logical Maximum (1)
    0x35, 0x00, //   Physical Minimum (0)
    0x45, 0x01, //   Physical Maximum (1)
    0x75, 0x01, //   Report Size (1)
    0x95, 0x06, //   Report Count (6)
    0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x95, 0x02, //   Report Count (2)
    0x81, 0x01, //   Input (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x75, 0x08, //   Report Size (8)
    0x95, 0x02, //   Report Count (2)
    0x81, 0x01, //   Input (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0xC0, // End Collection
];

const POWER_NOTCHES: [u8; 6] = [0x81, 0x6D, 0x54, 0x3F, 0x21, 0x00];
const BRAKE_NOTCHES: [u8; 10] = [0x79, 0x8A, 0x94, 0x9A, 0xA2, 0xA8, 0xAF, 0xB2, 0xB5, 0xB9];

bitflags! {
    struct Buttons: u8 {
        const NONE = 0;
        const B = 1;
        const A = 2;
        const C = 4;
        const D = 8;
        const SELECT = 16;
        const START = 32;
        const UP = Self::SELECT.bits | Self::D.bits;
        const DOWN = Self::SELECT.bits | Self::B.bits;
        const LEFT = Self::SELECT.bits | Self::A.bits;
        const RIGHT = Self::SELECT.bits | Self::C.bits;
    }
}

pub fn update_gadget(state: &mut ControllerState) {
    // Calculate data for handles
    let power = POWER_NOTCHES[state.power as usize];
    let brake = BRAKE_NOTCHES[state.brake as usize];

    // Calculate data for buttons
    let mut buttons = Buttons::NONE;
    if state.button_a {
        buttons.insert(Buttons::A)
    }
    if state.button_b {
        buttons.insert(Buttons::B)
    }
    if state.button_c {
        buttons.insert(Buttons::C)
    }
    if state.button_d {
        buttons.insert(Buttons::D)
    }
    if state.button_select {
        buttons.insert(Buttons::SELECT)
    }
    if state.button_start {
        buttons.insert(Buttons::START)
    }
    if state.button_up {
        buttons.insert(Buttons::UP)
    }
    if state.button_down {
        buttons.insert(Buttons::DOWN)
    }
    if state.button_left {
        buttons.insert(Buttons::LEFT)
    }
    if state.button_right {
        buttons.insert(Buttons::RIGHT)
    }

    // Assemble data and send it to gadget
    let data = [brake, power, 0, buttons.bits, 0, 0];
    if let Ok(mut file) = File::create(ENDPOINT1) {
        file.write(&data).ok();
    }
}
