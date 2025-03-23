use crate::controller::emulated::{DeviceDescriptor, ENDPOINT1};
use crate::controller::physical::ControllerState;
use bitflags::bitflags;
use std::fs::File;
use std::io::Write;

pub const DESCRIPTORS: [u8; 80] = [
    0x01, 0x00, 0x00, 0x00, 0x50, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
    0x09, 0x04, 0x00, 0x00, 0x02, 0x03, 0x00, 0x00, 0x00, 0x09, 0x21, 0x11, 0x01, 0x00, 0x01, 0x22,
    0x5E, 0x00, 0x07, 0x05, 0x02, 0x03, 0x40, 0x00, 0x05, 0x07, 0x05, 0x81, 0x03, 0x40, 0x00, 0x05,
    0x09, 0x04, 0x00, 0x00, 0x02, 0x03, 0x00, 0x00, 0x00, 0x09, 0x21, 0x11, 0x01, 0x00, 0x01, 0x22,
    0x5E, 0x00, 0x07, 0x05, 0x02, 0x03, 0x40, 0x00, 0x05, 0x07, 0x05, 0x81, 0x03, 0x40, 0x00, 0x05,
];
pub const STRINGS: [u8; 16] = [
    0x02, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub const DEVICE_DESCRIPTOR: DeviceDescriptor = DeviceDescriptor {
    b_device_class: 0x0,
    b_device_sub_class: 0x0,
    id_vendor: 0x33DD,
    id_product: 0x0001,
    bcd_device: 0x0106,
    i_manufacturer: "TAITO",
    i_product: "Densha de Go! Plug & Play (NS One Handle mode)",
    i_serial_number: "ZKNS-001",
};

pub const HID_REPORT_DESCRIPTOR: [u8; 94] = [
    0x05, 0x01, // Usage Page (Generic Desktop Ctrls)
    0x09, 0x05, // Usage (Game Pad)
    0xA1, 0x01, // Collection (Application)
    0x15, 0x00, //   Logical Minimum (0)
    0x25, 0x01, //   Logical Maximum (1)
    0x35, 0x00, //   Physical Minimum (0)
    0x45, 0x01, //   Physical Maximum (1)
    0x75, 0x01, //   Report Size (1)
    0x95, 0x0E, //   Report Count (14)
    0x05, 0x09, //   Usage Page (Button)
    0x19, 0x01, //   Usage Minimum (0x01)
    0x29, 0x0E, //   Usage Maximum (0x0E)
    0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x95, 0x02, //   Report Count (2)
    0x81, 0x01, //   Input (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x05, 0x01, //   Usage Page (Generic Desktop Ctrls)
    0x25, 0x07, //   Logical Maximum (7)
    0x46, 0x3B, 0x01, //   Physical Maximum (315)
    0x75, 0x04, //   Report Size (4)
    0x95, 0x01, //   Report Count (1)
    0x65, 0x14, //   Unit (System: English Rotation, Length: Centimeter)
    0x09, 0x39, //   Usage (Hat switch)
    0x81, 0x42, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,Null State)
    0x65, 0x00, //   Unit (None)
    0x95, 0x01, //   Report Count (1)
    0x81, 0x01, //   Input (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x26, 0xFF, 0x00, //   Logical Maximum (255)
    0x46, 0xFF, 0x00, //   Physical Maximum (255)
    0x09, 0x30, //   Usage (X)
    0x09, 0x31, //   Usage (Y)
    0x09, 0x32, //   Usage (Z)
    0x09, 0x35, //   Usage (Rz)
    0x75, 0x08, //   Report Size (8)
    0x95, 0x04, //   Report Count (4)
    0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x75, 0x08, //   Report Size (8)
    0x95, 0x01, //   Report Count (1)
    0x81, 0x01, //   Input (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x0A, 0x4F, 0x48, //   Usage (0x484F)
    0x75, 0x08, //   Report Size (8)
    0x95, 0x08, //   Report Count (8)
    0xB1,
    0x02, //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x0A, 0x4F, 0x48, //   Usage (0x484F)
    0x91,
    0x02, //   Output (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0xC0, // End Collection
];

const POWER_NOTCHES: [u8; 6] = [0x80, 0x9F, 0xB7, 0xCE, 0xE6, 0xFF];
const BRAKE_NOTCHES: [u8; 10] = [0x80, 0x65, 0x57, 0x49, 0x3C, 0x2E, 0x20, 0x13, 0x05, 0x00];

bitflags! {
    struct Buttons1: u8 {
        const NONE = 0;
        const Y = 1;
        const B = 2;
        const A = 4;
        const X = 8;
        const L = 16;
        const R = 32;
        const ZL = 64;
    }
    struct Buttons2: u8 {
        const NONE = 0;
        const SELECT = 1;
        const START = 2;
        const HOME = 16;
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
    if !state.button_select_hold && state.button_select && state.button_left {
        buttons1.insert(Buttons1::L);
        state.combo = true;
    }
    if !state.button_select_hold && state.button_select && state.button_right {
        buttons1.insert(Buttons1::R);
        state.combo = true;
    }
    if !state.button_select_hold && state.button_start && state.button_select {
        buttons2.insert(Buttons2::HOME);
        state.combo = true;
    }
    if state.button_a {
        buttons1.insert(Buttons1::Y)
    }
    if state.button_b {
        buttons1.insert(Buttons1::B)
    }
    if state.button_c {
        buttons1.insert(Buttons1::A)
    }
    if state.button_d {
        buttons1.insert(Buttons1::X)
    }
    if state.brake == 9 {
        buttons1.insert(Buttons1::ZL)
    }
    if !state.combo && state.button_start {
        buttons2.insert(Buttons2::START)
    }
    if !state.combo && state.button_select_hold {
        buttons2.insert(Buttons2::SELECT)
    }

    // Calculate data for D-pad
    let mut dpad: u8 = 0xF;
    if state.button_up {
        dpad = 0x0
    }
    if state.button_down {
        dpad = 0x4
    }
    if !state.combo && state.button_left {
        dpad = 0x6
    }
    if !state.combo && state.button_right {
        dpad = 0x2
    }
    if !state.combo && state.button_up & state.button_left {
        dpad = 0x7
    }
    if !state.combo && state.button_up & state.button_right {
        dpad = 0x1
    }
    if !state.combo && state.button_down & state.button_left {
        dpad = 0x5
    }
    if !state.combo && state.button_down & state.button_right {
        dpad = 0x3
    }

    // Assemble data and send it to gadget
    let data = [
        buttons1.bits,
        buttons2.bits,
        dpad,
        0x80,
        handle,
        0x80,
        0x80,
        0x00,
    ];
    if let Ok(mut file) = File::create(ENDPOINT1) {
        file.write(&data).ok();
    }
}
