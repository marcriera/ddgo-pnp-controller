use crate::controller::emulated::{DeviceDescriptor, ENDPOINT1};
use crate::controller::physical::ControllerState;
use bitflags::bitflags;
use std::fs::File;
use std::io::Write;

pub const DESCRIPTORS: [u8; 66] = [
    0x01, 0x00, 0x00, 0x00, 0x42, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
    0x09, 0x04, 0x00, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00,
    0x09, 0x21, 0x11, 0x01, 0x00, 0x01, 0x22, 0x33, 0x00,
    0x07, 0x05, 0x81, 0x03, 0x08, 0x00, 0x05,
    0x09, 0x04, 0x00, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00,
    0x09, 0x21, 0x11, 0x01, 0x00, 0x01, 0x22, 0x33, 0x00,
    0x07, 0x05, 0x81, 0x03, 0x08, 0x00, 0x05,
];
pub const STRINGS: [u8; 16] = [
    0x02, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub const DEVICE_DESCRIPTOR: DeviceDescriptor = DeviceDescriptor {
    b_device_class: 0x0,
    b_device_sub_class: 0x0,
    id_vendor: 0x1209,
    id_product: 0xD500,
    bcd_device: 0x0100,
    i_manufacturer: "TAITO",
    i_product: "Densha de Go! Plug & Play (Generic Train Controller mode)",
    i_serial_number: "GENERIC-TRAIN-CONTROLLER",
};

const POWER_NOTCHES: [u8; 6] = [0x80, 0x94, 0xAC, 0xCC, 0xE4, 0xFF];
const BRAKE_NOTCHES: [u8; 10] = [0x80, 0x91, 0x9F, 0xAD, 0xBB, 0xC9, 0xD7, 0xE5, 0xF3, 0xFF];

pub const HID_REPORT_DESCRIPTOR: [u8; 51] = [
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
    0x05, 0x09, //   Usage Page (Button)
    0x19, 0x01, //   Usage Minimum (0x01)
    0x29, 0x11, //   Usage Maximum (0x11)
    0x15, 0x00, //   Logical Minimum (0)
    0x25, 0x01, //   Logical Maximum (1)
    0x35, 0x00, //   Physical Minimum (0)
    0x45, 0x01, //   Physical Maximum (1)
    0x75, 0x01, //   Report Size (1)
    0x95, 0x11, //   Report Count (17)
    0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x95, 0x0F, //   Report Count (15)
    0x81, 0x01, //   Input (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0xC0, // End Collection
];

bitflags! {
    struct Buttons: u32 {
        const NONE = 0;
        const A = 0x1;
        const B = 0x2;
        const C = 0x4;
        const D = 0x8;
        const UP = 0x10;
        const DOWN = 0x20;
        const LEFT = 0x40;
        const RIGHT = 0x80;
        const SELECT = 0x100;
        const START = 0x200;
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
    if state.button_select {
        buttons.insert(Buttons::SELECT)
    }
    if state.button_start {
        buttons.insert(Buttons::START)
    }

    // Assemble data and send it to gadget
    let data = [
        brake,
        power,
        buttons.bits.to_le_bytes()[0],
        buttons.bits.to_le_bytes()[1],
        buttons.bits.to_le_bytes()[2],
        buttons.bits.to_le_bytes()[3],
    ];
    if let Ok(mut file) = File::create(ENDPOINT1) {
        file.write(&data).ok();
    }
}
