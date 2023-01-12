
use macroquad::prelude::*;

use crate::game_state::{GameState, Rotation, Direction};

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
    pub das: f64,
    pub arr: f64,
    pub are: f64,
    pub das_sd: f64,
    pub arr_sd: f64,
    pub das_type: DasType,
}

impl Handling {
    pub fn default() -> Self {
        Handling {
            das: 0.167,
            arr: 0.033,
            are: 0.0,
            das_sd: 0.167,
            arr_sd: 0.033,
            das_type: DasType::Default,
        }
    }
}

pub enum DasType {
    Default,
}

pub struct InputProcessor {
    pub controls: Controls,
    pub handling: Handling,
    pub moves_side: isize,
    pub moves_down: isize,
    pub das_left: f64,
    pub das_right: f64,
    pub das_sd: f64,
}

impl InputProcessor {
    pub fn new(controls: Controls, handling: Handling) -> Self {
        InputProcessor {
            controls,
            handling,
            moves_down: 0,
            moves_side: 0,
            das_left: -1f64,
            das_right: -1f64,
            das_sd: -1f64,
        }
    }

    pub fn update_state(&mut self, game_state: &mut GameState) {
        let time_now = get_time();
        self.process_das(time_now, game_state);

        // one-time inputs
        if is_key_pressed(self.controls.hold) {
            self.reset_on_new(time_now);
            game_state.hold();
        }
        if is_key_pressed(self.controls.hard_drop) {
            self.reset_on_new(time_now);
            game_state.hard_drop();
        }
        if is_key_pressed(self.controls.reset) {
            self.reset_on_new(time_now);
            game_state.reset();
        }

        // rotations
        if is_key_pressed(self.controls.rotate_ccw) {
            game_state.rotate(Rotation::Ccw);
        }
        if is_key_pressed(self.controls.rotate_cw) {
            game_state.rotate(Rotation::Cw);
        }
        if is_key_pressed(self.controls.rotate_ccw_120) {
            game_state.rotate(Rotation::Ccw120);
        }
        if is_key_pressed(self.controls.rotate_cw_120) {
            game_state.rotate(Rotation::Cw120);
        }
        if is_key_pressed(self.controls.rotate_180) {
            game_state.rotate(Rotation::Center);
        }
    }

    pub fn reset_on_new(&mut self, time_now: f64) {
        if self.das_left > 0f64 {
            self.das_left = time_now - self.handling.das;
            self.moves_side = 0;
        }
        if self.das_right > 0f64 {
            self.das_right = time_now - self.handling.das;
            self.moves_side = 0;
        }
        if self.das_sd > 0f64 {
            self.das_sd = time_now - self.handling.das_sd;
            self.moves_down = 0;
        }
    }

    pub fn process_das(&mut self, time_now: f64, game_state: &mut GameState) {
        match self.handling.das_type {
            DasType::Default => {
                // softdrop
                if is_key_pressed(self.controls.soft_drop) {
                    game_state.move_down();
                    self.das_sd = time_now;
                    self.moves_down = 1;
                }
                if is_key_released(self.controls.soft_drop) {
                    self.das_sd = -1f64;
                }

                // movement key down
                if is_key_pressed(self.controls.move_left) {
                    game_state.move_left();
                    self.das_right = -1f64;
                    self.moves_side = 0;
                    self.das_left = time_now;
                }
                if is_key_pressed(self.controls.move_right) {
                    game_state.move_right();
                    self.das_left = -1f64;
                    self.moves_side = 0;
                    self.das_right = time_now;
                }

                // movement key up
                if is_key_released(self.controls.move_right) {
                    self.das_right = -1f64;
                    if is_key_down(self.controls.move_left) {
                        self.das_left = time_now;
                        self.moves_side = 0;
                    }
                }
                if is_key_released(self.controls.move_left) {
                    self.das_left = -1f64;
                    if is_key_down(self.controls.move_right) {
                        self.das_right = time_now;
                        self.moves_side = 0;
                    }
                }

                // movement processing
                let (current_side_das, move_direction) = if self.das_left > 0f64 {
                    (self.das_left, Direction::Left)
                } else {
                    (self.das_right, Direction::Right)
                };
                if current_side_das > 0f64 {
                    let duration = time_now - current_side_das - self.handling.das;
                    if self.handling.arr == 0f64 {
                        if duration > 0f64 {
                            while game_state.move_direction(&move_direction) {
                                self.moves_side += 1;
                            }
                        }
                    } else {
                        let mut expected_moves = (duration / self.handling.arr) as isize;
                        if duration > 0f64 {
                            if self.moves_side == 0 {
                                game_state.move_direction(&move_direction);
                                self.moves_side += 1;
                            }
                            expected_moves += 1;
                        }
                        for _ in 0..(expected_moves - self.moves_side) {
                            if !game_state.move_direction(&move_direction) {
                                break;
                            }
                            self.moves_side += 1;
                        }
                    }
                }

                // softdrop processing
                if self.das_sd > 0f64 {
                    let duration = time_now - self.das_sd - self.handling.das_sd;
                    if self.handling.arr_sd == 0f64 {
                        if duration > 0f64 {
                            while game_state.move_down() {
                                self.moves_down += 1;
                            }
                        }
                    } else {
                        let mut expected_moves = (duration / self.handling.arr_sd) as isize + 1;
                        if duration > 0f64 {
                            if self.moves_down < 2 {
                                game_state.move_down();
                                self.moves_down += 1;
                            }
                            expected_moves += 1;
                        }
                        for _ in 0..(expected_moves - self.moves_down) {
                            if !game_state.move_down() {
                                break;
                            }
                            self.moves_down += 1;
                        }
                    }
                }
            }
        }
        // let das_direction_left = self.das_left > 0f64;
        // let current_das = if das_direction_left {self.das_left} else {self.das_right};
        // if self.das_left > 0f64 || self.das_right > 0f64 {
        //     let mut das_duration = time_now - current_das;
        //     let mut supposed_moved = 0;
        //     if das_duration > self.handling.das {
        //         das_duration -= self.handling.das;
        //         supposed_moved += if self.handling.arr > 0f64 {
        //             (das_duration / self.handling.arr) as isize
        //         } else {
        //             game_state.matrix.width as isize
        //         };
        //     }
        //     for _ in 0..(supposed_moved - self.moved as isize) {
        //         self.moved += 1;
        //         if das_direction_left{
        //             game_state.move_left();
        //         } else {
        //             game_state.move_right();
        //         }
        //     }
        // }
    }
}