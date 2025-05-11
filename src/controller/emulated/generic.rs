use crate::controller::emulated::{DeviceDescriptor, ENDPOINT0, ENDPOINT1};
use crate::controller::physical::ControllerState;
use bitflags::bitflags;
use std::fs::File;
use std::io::Write;

pub const DESCRIPTORS: [u8; 80] = [
    0x01, 0x00, 0x00, 0x00, 0x50, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
    0x09, 0x04, 0x00, 0x00, 0x02, 0x03, 0x00, 0x00, 0x00, 0x09, 0x21, 0x11, 0x01, 0x00, 0x01, 0x22,
    0x94, 0x00, 0x07, 0x05, 0x02, 0x03, 0x40, 0x00, 0x05, 0x07, 0x05, 0x81, 0x03, 0x40, 0x00, 0x05,
    0x09, 0x04, 0x00, 0x00, 0x02, 0x03, 0x00, 0x00, 0x00, 0x09, 0x21, 0x11, 0x01, 0x00, 0x01, 0x22,
    0x94, 0x00, 0x07, 0x05, 0x02, 0x03, 0x40, 0x00, 0x05, 0x07, 0x05, 0x81, 0x03, 0x40, 0x00, 0x05,
];
pub const STRINGS: [u8; 16] = [
    0x02, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub const DEVICE_DESCRIPTOR: DeviceDescriptor = DeviceDescriptor {
    b_device_class: 0x0,
    b_device_sub_class: 0x0,
    id_vendor: 0x054C,
    id_product: 0x0268,
    bcd_device: 0x0100,
    i_manufacturer: "TAITO",
    i_product: "Densha de Go! Plug & Play (Generic Train Controller mode)",
    i_serial_number: "GENERIC-TRAIN-CONTROLLER",
};

const POWER_NOTCHES: [u8; 6] = [0x80, 0x94, 0xAC, 0xCC, 0xE4, 0xFF];
const BRAKE_NOTCHES: [u8; 10] = [0x80, 0x91, 0x9F, 0xAD, 0xBB, 0xC9, 0xD7, 0xE5, 0xF3, 0xFF];

pub const HID_REPORT_DESCRIPTOR: [u8; 148] = [
    0x05, 0x01, // Usage Page (Generic Desktop Ctrls)
    0x09, 0x04, // Usage (Joystick)
    0xA1, 0x01, // Collection (Physical)
    0xA1, 0x02, //   Collection (Application)
    0x85, 0x01, //     Report ID (1)
    0x75, 0x08, //     Report Size (8)
    0x95, 0x01, //     Report Count (1)
    0x15, 0x00, //     Logical Minimum (0)
    0x26, 0xFF, 0x00, //     Logical Maximum (255)
    0x81, 0x03, //     Input (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    //     NOTE: reserved byte
    0x75, 0x01, //     Report Size (1)
    0x95, 0x13, //     Report Count (19)
    0x15, 0x00, //     Logical Minimum (0)
    0x25, 0x01, //     Logical Maximum (1)
    0x35, 0x00, //     Physical Minimum (0)
    0x45, 0x01, //     Physical Maximum (1)
    0x05, 0x09, //     Usage Page (Button)
    0x19, 0x01, //     Usage Minimum (0x01)
    0x29, 0x13, //     Usage Maximum (0x13)
    0x81, 0x02, //     Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x75, 0x01, //     Report Size (1)
    0x95, 0x0D, //     Report Count (13)
    0x06, 0x00, 0xFF, //     Usage Page (Vendor Defined 0xFF00)
    0x81, 0x03, //     Input (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    //     NOTE: 32 bit integer, where 0:18 are buttons and 19:31 are reserved
    0x15, 0x00, //     Logical Minimum (0)
    0x26, 0xFF, 0x00, //     Logical Maximum (255)
    0x05, 0x01, //     Usage Page (Generic Desktop Ctrls)
    0x09, 0x01, //     Usage (Pointer)
    0xA1, 0x00, //     Collection (Undefined)
    0x75, 0x08, //       Report Size (8)
    0x95, 0x04, //       Report Count (4)
    0x35, 0x00, //       Physical Minimum (0)
    0x46, 0xFF, 0x00, //       Physical Maximum (255)
    0x09, 0x30, //       Usage (X)
    0x09, 0x31, //       Usage (Y)
    0x09, 0x32, //       Usage (Z)
    0x09, 0x35, //       Usage (Rz)
    0x81, 0x02, //       Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    //       NOTE: four joysticks
    0xC0, //     End Collection
    0x05, 0x01, //     Usage Page (Generic Desktop Ctrls)
    0x75, 0x08, //     Report Size (8)
    0x95, 0x27, //     Report Count (39)
    0x09, 0x01, //     Usage (Pointer)
    0x81, 0x02, //     Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x75, 0x08, //     Report Size (8)
    0x95, 0x30, //     Report Count (48)
    0x09, 0x01, //     Usage (Pointer)
    0x91,
    0x02, //     Output (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x75, 0x08, //     Report Size (8)
    0x95, 0x30, //     Report Count (48)
    0x09, 0x01, //     Usage (Pointer)
    0xB1,
    0x02, //     Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0xC0, //   End Collection
    0xA1, 0x02, //   Collection (Application)
    0x85, 0x02, //     Report ID (2)
    0x75, 0x08, //     Report Size (8)
    0x95, 0x30, //     Report Count (48)
    0x09, 0x01, //     Usage (Pointer)
    0xB1,
    0x02, //     Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0xC0, //   End Collection
    0xA1, 0x02, //   Collection (Application)
    0x85, 0xEE, //     Report ID (238)
    0x75, 0x08, //     Report Size (8)
    0x95, 0x30, //     Report Count (48)
    0x09, 0x01, //     Usage (Pointer)
    0xB1,
    0x02, //     Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0xC0, //   End Collection
    0xA1, 0x02, //   Collection (Application)
    0x85, 0xEF, //     Report ID (239)
    0x75, 0x08, //     Report Size (8)
    0x95, 0x30, //     Report Count (48)
    0x09, 0x01, //     Usage (Pointer)
    0xB1,
    0x02, //     Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0xC0, //   End Collection
    0xC0, // End Collection
];

const F2_REPORT: [u8; 64] = [
    0xF2, 0xFF, 0xFF, 0x0, 0x0, 0x6, 0xF5, 0x48, 0xE2, 0x49, 0x0, 0x3, 0x50, 0x81, 0xD8, 0x1, 0x8A,
    0x13, 0x0, 0x0, 0x0, 0x0, 0x4, 0x0, 0x2, 0x2, 0x2, 0x2, 0x0, 0x0, 0x0, 0x4, 0x4, 0x4, 0x4, 0x0,
    0x0, 0x4, 0x0, 0x1, 0x2, 0x7, 0x0, 0x17, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];

const F5_REPORT: [u8; 64] = [
    0x1, 0x0, 0x0, 0x23, 0x6, 0x7C, 0xB9, 0xB, 0xE2, 0x49, 0x0, 0x3, 0x50, 0x81, 0xD8, 0x1, 0x8A,
    0x13, 0x0, 0x0, 0x0, 0x0, 0x4, 0x0, 0x2, 0x2, 0x2, 0x2, 0x0, 0x0, 0x0, 0x4, 0x4, 0x4, 0x4, 0x0,
    0x0, 0x4, 0x0, 0x1, 0x2, 0x7, 0x0, 0x17, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];

bitflags! {
    struct Buttons1: u8 {
        const NONE = 0;
        const SELECT = 1;
        const L3 = 2;
        const R3 = 4;
        const START = 8;
        const UP = 16;
        const RIGHT = 32;
        const DOWN = 64;
        const LEFT = 128;
    }
    struct Buttons2: u8 {
        const NONE = 0;
        const L2 = 1;
        const R2 = 2;
        const L1 = 4;
        const R1 = 8;
        const TRIANGLE = 16;
        const CIRCLE = 32;
        const CROSS = 64;
        const SQUARE = 128;
    }
}

pub fn update_gadget(state: &mut ControllerState) {
    // Calculate data for handles
    let power = POWER_NOTCHES[state.power as usize];
    let brake = BRAKE_NOTCHES[state.brake as usize];

    // Calculate data for buttons
    let mut buttons1 = Buttons1::NONE;
    let mut buttons2 = Buttons2::NONE;

    if state.button_a {
        buttons2.insert(Buttons2::SQUARE)
    }
    if state.button_b {
        buttons2.insert(Buttons2::CROSS)
    }
    if state.button_c {
        buttons2.insert(Buttons2::CIRCLE)
    }
    if state.button_d {
        buttons2.insert(Buttons2::TRIANGLE)
    }
    if state.button_start {
        buttons1.insert(Buttons1::START)
    }
    if state.button_select {
        buttons1.insert(Buttons1::SELECT)
    }
    if state.button_up {
        buttons1.insert(Buttons1::UP)
    }
    if state.button_down {
        buttons1.insert(Buttons1::DOWN)
    }
    if state.button_left {
        buttons1.insert(Buttons1::LEFT)
    }
    if state.button_right {
        buttons1.insert(Buttons1::RIGHT)
    }

    let btn_up = if buttons1.contains(Buttons1::UP) {
        0xFF
    } else {
        0x0
    };
    let btn_right = if buttons1.contains(Buttons1::RIGHT) {
        0xFF
    } else {
        0x0
    };
    let btn_down = if buttons1.contains(Buttons1::DOWN) {
        0xFF
    } else {
        0x0
    };
    let btn_left = if buttons1.contains(Buttons1::LEFT) {
        0xFF
    } else {
        0x0
    };
    let btn_triangle = if buttons2.contains(Buttons2::TRIANGLE) {
        0xFF
    } else {
        0x0
    };
    let btn_circle = if buttons2.contains(Buttons2::CIRCLE) {
        0xFF
    } else {
        0x0
    };
    let btn_cross = if buttons2.contains(Buttons2::CROSS) {
        0xFF
    } else {
        0x0
    };
    let btn_square = if buttons2.contains(Buttons2::SQUARE) {
        0xFF
    } else {
        0x0
    };

    // Assemble data and send it to gadget
    let data = [
        0x1,
        0x0,
        buttons1.bits,
        buttons2.bits,
        0x0,
        0x0,
        power,
        brake,
        0x80,
        0x80,
        0x0,
        0x0,
        0x0,
        0x0,
        btn_up,
        btn_right,
        btn_down,
        btn_left,
        0x0,
        0x0,
        0x0,
        0x0,
        btn_triangle,
        btn_circle,
        btn_cross,
        btn_square,
        0x0,
        0x0,
        0x0,
        0x3,
        0xEF,
        0x14,
        0x0,
        0x0,
        0x0,
        0x0,
        0x23,
        0x1A,
        0x77,
        0x1,
        0x81,
        0x1,
        0xFE,
        0x1,
        0xFE,
        0x1,
        0xFE,
        0x1,
        0xFE,
    ];
    if let Ok(mut file) = File::create(ENDPOINT1) {
        file.write(&data).ok();
    }
}

pub fn handle_ctrl_transfer(data: &[u8]) {
    if data[1] == 1 && data[2] == 0xF2 {
        // Init 1
        if let Ok(mut file) = File::create(ENDPOINT0) {
            file.write(&F2_REPORT).unwrap();
        }
    } else if data[1] == 1 && data[2] == 0xF5 {
        // Init 2
        if let Ok(mut file) = File::create(ENDPOINT0) {
            file.write(&F5_REPORT).unwrap();
        }
    }
}
