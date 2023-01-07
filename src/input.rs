
use macroquad::prelude::*;

pub struct Controls {
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub soft_drop: KeyCode,
    pub hard_drop: KeyCode,
    pub rotate_ccw: KeyCode,
    pub rotate_cw: KeyCode,
    pub rotate_ccw_120: KeyCode,
    pub rotate_cw_120: KeyCode,
    pub rotate_180: KeyCode,
    pub hold: KeyCode,
    pub reset: KeyCode,
}

impl Controls {
    pub fn default() -> Self {
        Controls {
            move_left: KeyCode::Left,
            move_right: KeyCode::Right,
            soft_drop: KeyCode::Down,
            hard_drop: KeyCode::Space,
            rotate_ccw: KeyCode::Z,
            rotate_cw: KeyCode::C,
            rotate_ccw_120: KeyCode::A,
            rotate_cw_120: KeyCode::D,
            rotate_180: KeyCode::X,
            hold: KeyCode::Up,
            reset: KeyCode::R,
        }
    }
}

pub struct Handling {
    pub das: u32,
    pub arr: u32,
    pub are: u32,
    pub sd_das: u32,
    pub sd_arr: u32,
}

impl Handling {
    pub fn default() -> Self {
        Handling {
            das: 70,
            arr: 0,
            are: 0,
            sd_das: 70,
            sd_arr: 0,
        }
    }
}

pub struct InputProcessor {

}

impl InputProcessor {
    pub fn update_state() {
        
    }
}