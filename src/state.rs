#[derive(Default)]
pub struct ControllerState {
    pub power: u8,
    pub brake: u8,
    pub button_select: bool,
    pub button_start: bool,
    pub button_a: bool,
    pub button_b: bool,
    pub button_c: bool,
    pub button_d: bool,
    pub button_up: bool,
    pub button_down: bool,
    pub button_left: bool,
    pub button_right: bool,
}