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
    bcd_device: 0x0500,
    i_manufacturer: "TAITO",
    i_product: "Densha de Go! Plug & Play (DC Two Handle mode)",
    i_serial_number: "TCPP-20004",
};

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
        const BRAKE1 = 0x400;
        const BRAKE2 = 0x800;
        const BRAKE3 = 0x1000;
        const BRAKE4 = 0x2000;
        const POWER1 = 0x4000;
        const POWER2 = 0x8000;
        const POWER3 = 0x10000;
    }
}

pub fn update_gadget(state: &mut ControllerState) {
    let mut buttons = Buttons::NONE;

    // Calculate data for handles
    match state.power {
        0 => {
            buttons.insert(Buttons::POWER2 | Buttons::POWER3);
        }
        1 => {
            buttons.insert(Buttons::POWER1 | Buttons::POWER3);
        }
        2 => {
            buttons.insert(Buttons::POWER3);
        }
        3 => {
            buttons.insert(Buttons::POWER1 | Buttons::POWER2);
        }
        4 => {
            buttons.insert(Buttons::POWER2);
        }
        _ => {
            buttons.insert(Buttons::POWER1);
        }
    }
    match state.brake {
        0 => {
            buttons.insert(Buttons::BRAKE2 | Buttons::BRAKE3 | Buttons::BRAKE4);
        }
        1 => {
            buttons.insert(Buttons::BRAKE1 | Buttons::BRAKE3 | Buttons::BRAKE4);
        }
        2 => {
            buttons.insert(Buttons::BRAKE3 | Buttons::BRAKE4);
        }
        3 => {
            buttons.insert(Buttons::BRAKE1 | Buttons::BRAKE2 | Buttons::BRAKE4);
        }
        4 => {
            buttons.insert(Buttons::BRAKE2 | Buttons::BRAKE4);
        }
        5 => {
            buttons.insert(Buttons::BRAKE1 | Buttons::BRAKE4);
        }
        6 => {
            buttons.insert(Buttons::BRAKE4);
        }
        7 => {
            buttons.insert(Buttons::BRAKE1 | Buttons::BRAKE2 | Buttons::BRAKE3);
        }
        8 => {
            buttons.insert(Buttons::BRAKE2 | Buttons::BRAKE3);
        }
        _ => (),
    }

    // Calculate data for buttons
    if state.button_a {
        buttons.insert(Buttons::A)
    }
    if state.button_b {
        buttons.insert(Buttons::B)
    }
    if state.button_c {
        buttons.insert(Buttons::C)
    }
    if state.button_start {
        buttons.insert(Buttons::START)
    }
    if state.button_select {
        buttons.insert(Buttons::SELECT)
    }

    // Assemble data and send it to gadget
    let data = [
        0x80,
        0x80,
        buttons.bits.to_le_bytes()[0],
        buttons.bits.to_le_bytes()[1],
        buttons.bits.to_le_bytes()[2],
        buttons.bits.to_le_bytes()[3],
    ];
    if let Ok(mut file) = File::create(ENDPOINT1) {
        file.write(&data).ok();
    }
}
