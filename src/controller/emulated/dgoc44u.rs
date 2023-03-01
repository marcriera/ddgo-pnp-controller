use crate::state::ControllerState;

const POWER_NOTCHES: [u8; 6] = [0x81, 0x6D, 0x54, 0x3F, 0x21, 0x00];
const BRAKE_NOTCHES: [u8; 10] = [0x79, 0x8A, 0x94, 0x9A, 0xA2, 0xA8, 0xAF, 0xB2, 0xB5, 0xB9];

pub fn update_gadget(state: &mut ControllerState) {
        // Calculate data for handles
        let power = POWER_NOTCHES[state.power as usize];
        let brake = BRAKE_NOTCHES[state.brake as usize];

        // Assemble data and send it to gadget
        let data = [brake, power, 0, 0, 0, 0];
}