use std::fs::File;
use std::io::Write;
use std::io::Result;
use std::time::{Instant,Duration};

use evdev::Device;
use evdev::Key;

#[derive(Default)]
pub struct ControllerState {
    pub power: u8,
    pub brake: u8,
    pub button_select: bool,
    pub button_select_hold: bool,
    pub button_select_time: Option<Instant>,
    pub button_start: bool,
    pub button_a: bool,
    pub button_b: bool,
    pub button_c: bool,
    pub button_d: bool,
    pub button_up: bool,
    pub button_down: bool,
    pub button_left: bool,
    pub button_right: bool,
    pub lamp: bool,
    pub rumble: bool,
    pub combo: bool,
    pub reverser: u8,
}

const HOLD_DELAY: Duration = Duration::from_millis(750);

const USED_KEYS: [Key; 26] = [Key::KEY_0, Key::KEY_1, Key::KEY_2, Key::KEY_3, Key::KEY_4, Key::KEY_5,
                            Key::KEY_B, Key::KEY_C, Key::KEY_D, Key::KEY_E, Key::KEY_F, Key::KEY_G, Key::KEY_H, Key::KEY_I, Key::KEY_J, Key::KEY_P,
                            Key::KEY_SPACE, Key::KEY_ENTER, Key::KEY_A, Key::KEY_Z, Key::KEY_X, Key::KEY_S, Key::KEY_UP, Key::KEY_DOWN, Key::KEY_LEFT, Key::KEY_RIGHT];

pub fn init() -> Result<[Device; 2]> {
    let d1 = Device::open("/dev/input/event1")?;
    let d2 = Device::open("/dev/input/event2")?;
    Ok([d1, d2])
}

pub fn get_state(state: &mut ControllerState, dev: &[Device; 2]) {
    for d in dev {
        if let Ok(key_vals) = d.get_key_state() {
            for key in USED_KEYS {
                if d.supported_keys().map_or(true, |k| k.contains(key)) {
                    read_input(state, key, key_vals.contains(key));
                }
            }
        }
    }
}

fn read_input(controller: &mut ControllerState, key: Key, value: bool) {
    // Save input status to object for processing
    match key {
        Key::KEY_0=>if value {controller.power = 0},
        Key::KEY_1=>if value {controller.power = 1},
        Key::KEY_2=>if value {controller.power = 2},
        Key::KEY_3=>if value {controller.power = 3},
        Key::KEY_4=>if value {controller.power = 4},
        Key::KEY_5=>if value {controller.power = 5},
        Key::KEY_B=>if value {controller.brake = 0},
        Key::KEY_C=>if value {controller.brake = 1},
        Key::KEY_D=>if value {controller.brake = 2},
        Key::KEY_E=>if value {controller.brake = 3},
        Key::KEY_F=>if value {controller.brake = 4},
        Key::KEY_G=>if value {controller.brake = 5},
        Key::KEY_H=>if value {controller.brake = 6},
        Key::KEY_I=>if value {controller.brake = 7},
        Key::KEY_J=>if value {controller.brake = 8},
        Key::KEY_P=>if value {controller.brake = 9},
        Key::KEY_SPACE=> {
            if !controller.button_select && value {
                controller.button_select_time = Some(Instant::now());
            }
            else if !value {
                controller.button_select_time = None;
                controller.combo = false;
            }
            controller.button_select = value;
            controller.button_select_hold = value;
            if let Some(time) = controller.button_select_time {
                controller.button_select_hold = time.elapsed() > HOLD_DELAY && !controller.combo;
            }
        },
        Key::KEY_ENTER=>controller.button_start = value,
        Key::KEY_A=>controller.button_a = value,
        Key::KEY_Z=>controller.button_b = value,
        Key::KEY_X=>controller.button_c = value,
        Key::KEY_S=>controller.button_d = value,
        Key::KEY_UP=>controller.button_up = value,
        Key::KEY_DOWN=>controller.button_down = value,
        Key::KEY_LEFT=>controller.button_left = value,
        Key::KEY_RIGHT=>controller.button_right = value,
        _=>(),
    }
}

pub fn set_lamp(status: bool) {
    if let Ok(mut out) = File::create("/sys/class/leds/led2/brightness") {
        out.write(if status {b"1"} else {b"0"}).ok();
    }
    /*else {
        println!("WARNING: Could not set door lamp status!")
    }*/
}

pub fn set_rumble(status: bool) {
    if let Ok(mut out) = File::create("/sys/class/leds/led1/brightness") {
        out.write(if status {b"1"} else {b"0"}).ok();
    }
    /*else {
        println!("WARNING: Could not set rumble motor status!")
    }*/
}
