use bitflags::bitflags;
use crate::state::ControllerState;

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

    // Assemble data and send it to gadget
    let data = [0x1, brake, power, 0xFF, dpad, buttons.bits];
}