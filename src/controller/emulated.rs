use crate::state::ControllerState;
mod dgoc44u;

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
    else if state.button_up {
        println!("Selected controller TCPP-20009.");
        return ControllerModel::TYPE2;
    }
    else {
        println!("No controller selected.");
        return ControllerModel::NONE;
    }
}

pub fn set_state(state: &mut ControllerState, model: &ControllerModel) {
    match model {
        ControllerModel::DGOC44U => {
            dgoc44u::update_gadget(state);
        }
        ControllerModel::TYPE2 => {

        }
        ControllerModel::NONE => (),
    }
}
