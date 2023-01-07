
use macroquad::prelude::*;

pub struct ControlSetting {
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub soft_drop: KeyCode,
    pub hard_drop: KeyCode,
    pub rotate_ccw: KeyCode, // ccw
    pub rotate_cw: KeyCode, // cw
    pub rotate_ccw_120: KeyCode, // ccw 120
    pub rotate_cw_120: KeyCode, // cw 120
    pub rotate_180: KeyCode,
    pub hold: KeyCode,
    pub reset: KeyCode,
    pub das: u32,
    pub arr: u32,
    pub are: u32,
    pub sd_das: u32,
    pub sd_arr: u32,
}

impl ControlSetting {
    pub fn default() -> Self {
        ControlSetting {
            move_left: KeyCode::Left,
            move_right: KeyCode::Right,
            soft_drop: KeyCode::Down,
            hard_drop: KeyCode::Space,
            rotate_ccw: KeyCode::Z, // ccw
            rotate_cw: KeyCode::C, // cw
            rotate_ccw_120: KeyCode::A, // ccw 120
            rotate_cw_120: KeyCode::D, // cw 120
            rotate_180: KeyCode::X,
            hold: KeyCode::Up,
            reset: KeyCode::R,
            das: 70,
            arr: 0,
            are: 0,
            sd_das: 70,
            sd_arr: 0,
        }
    }
}