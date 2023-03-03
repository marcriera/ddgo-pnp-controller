use crate::state::ControllerState;
mod dgoc44u;
mod tcpp20009;
mod vok00106;

#[derive(PartialEq)]
pub enum ControllerModel {
    NONE,
    DGOC44U,
    TCPP20009,
    VOK00106,
}

pub fn set_model(state: &ControllerState) -> ControllerModel {
    if state.button_right {
        println!("Selected controller DGOC44-U.");
        return ControllerModel::DGOC44U;
    }
    else if state.button_up {
        println!("Selected controller TCPP-20009.");
        return ControllerModel::TCPP20009;
    }
    else if state.button_a {
        println!("Selected controller VOK-00106.");
        return ControllerModel::VOK00106;
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
        ControllerModel::TCPP20009 => {
            tcpp20009::update_gadget(state);
        }
        ControllerModel::VOK00106 => {
            vok00106::update_gadget(state);
        }
        _ => (),
    }
}
