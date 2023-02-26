use crate::state::ControllerState;

#[derive(PartialEq)]
pub enum ControllerModel {
    NONE,
    DGOC44U,
    TYPE2,
}

pub fn set_model(state: &ControllerState) -> ControllerModel {
    if state.button_right {
        println!("Selected controller DGOC44-U.");
        return ControllerModel::DGOC44U;
    }
    else if state.button_left {
        println!("Selected controller TCPP-20009.");
        return ControllerModel::TYPE2;
    }
    else {
        println!("No controller selected.");
        return ControllerModel::NONE;
    }
}