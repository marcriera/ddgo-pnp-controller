use crate::state::ControllerState;

const POWER_NOTCHES: [&str; 6] = ["TSA50", "TSA55", "TSA65", "TSA75", "TSA85", "TSA95"];
const BRAKE_NOTCHES: [&str; 10] = ["TSA50", "TSA45", "TSA35", "TSA25", "TSA15", "TSA05", "TSE99", "TSB40", "TSB30", "TSB20"];

pub fn update_gadget(state: &mut ControllerState) {
    // Calculate data for handles
    let power = POWER_NOTCHES[state.power as usize];
    let brake = BRAKE_NOTCHES[state.brake as usize];
}