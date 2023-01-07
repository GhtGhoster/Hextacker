
mod game_state;
mod hex;
mod input;

use game_state::*;
use hex::*;
//use control::*;

use std::collections::HashMap;
use input::Controls;

use macroquad::prelude::*;

#[macroquad::main("Hextacker")]
async fn main() {
    let mut piece_colors = HashMap::new();
    piece_colors.insert('s', Color::from_rgba(0  , 255, 0  , 255));
    piece_colors.insert('z', Color::from_rgba(255, 0  , 0  , 255));
    piece_colors.insert('b', Color::from_rgba(255, 127, 127, 255));
    piece_colors.insert('d', Color::from_rgba(0  , 191, 0  , 255));
    piece_colors.insert('j', Color::from_rgba(255, 127 , 0 , 255));
    piece_colors.insert('l', Color::from_rgba(0  , 0  , 255, 255));
    piece_colors.insert('c', Color::from_rgba(255, 0  , 127, 255));
    piece_colors.insert('t', Color::from_rgba(255, 0  , 255, 255));
    piece_colors.insert('o', Color::from_rgba(255, 255, 0  , 255));
    piece_colors.insert('i', Color::from_rgba(0  , 191, 191, 255));

    // let mut piece_shapes = HashMap::new();
    // piece_shapes.insert('s', Piece::new([(-1, -1), (-1, 0), (0, 0), (0, 1)], 's'));
    // piece_shapes.insert('z', Piece::new([(1, -2), (1, -1), (0, 0), (0, 1)], 'z'));
    // piece_shapes.insert('b', Piece::new([(0, -1), (0, 0), (0, 1), (1, 0)], 'b'));
    // piece_shapes.insert('d', Piece::new([(0, -1), (0, 0), (0, 1), (-1, 1)], 'd'));
    // piece_shapes.insert('j', Piece::new([(0, -2), (0, -1), (0, 0), (-1, 1)], 'j'));
    // piece_shapes.insert('l', Piece::new([(0, -2), (0, -1), (0, 0), (1, 0)], 'l'));
    // piece_shapes.insert('c', Piece::new([(-1, 1), (-1, 0), (0, -1), (1, -1)], 'c'));
    // piece_shapes.insert('t', Piece::new([(0, -1), (0, 0), (-1, 1), (1, 0)], 't'));
    // piece_shapes.insert('o', Piece::new([(0, -1), (0, 0), (-1, 0), (1, -1)], 'o'));
    // piece_shapes.insert('i', Piece::new([(0, -2), (0, -1), (0, 0), (0, 1)], 'i'));

    // let rng = &mut ::rand::thread_rng();

    // let pieces: Vec<char> = "szdbjliotc".chars().collect();
    // let mut piece_index: isize = rng.gen_range(0..pieces.len()) as isize;

    // let mut matrix = HexField::new(10, 40, 20);
    // let mut curr_piece = piece_shapes.get(pieces.get(piece_index as usize).unwrap()).unwrap().clone();
    // let mut hold_piece = Piece::new([(0, 0), (0, 0), (0, 0), (0, 0)], ' ');
    // curr_piece.position = (4, matrix.height as isize - matrix.playable_height as isize - 2);

    let (width, height, playable_height) = (10, 40, 20);
    let queue_view = 4;

    let mut game_state = GameState::new(width, height, playable_height);

    let controls = Controls::default();

    let background = BLACK;
    let border = DARKGRAY;

    loop {

        // calc flat-right
        let render_size = screen_height().min(screen_width()); // TODO: implement after UI
        let border_size = render_size/500f32;

        let board_height = render_size*0.95;
        let hex_size = ((board_height - (border_size * 2f32)) / ((playable_height*2) + (width-1)) as f32) / (SQRT_THREE/2f32);
        let tri_width = hex_size;
        let tri_height = hex_size * 0.5 * SQRT_THREE;
        let board_width = hex_size * ((3f32 * (width as f32 / 2f32)) + 0.5);

        let offset_x = (screen_width()-board_width) / 2f32;
        let offset_y = (screen_height()-board_height) / 2f32;

        // collide / input
        // let mut mouse_pos = mouse_position();
        // mouse_pos.0 -= offset_x + (border_size/2f32) + tri_width;
        // mouse_pos.1 -= offset_y + (border_size/2f32) + tri_height;
        // let (_mouse_q, _mouse_r) = pixel_to_flat_hex(mouse_pos, hex_size);

        // let (_mouse_wheel_x, mouse_wheel_y) = mouse_wheel();
        // if mouse_wheel_y != 0f32 {
        //     piece_index += if mouse_wheel_y < 0f32 {
        //         -1
        //     } else {
        //         1
        //     };
        //     while piece_index >= pieces.len() as isize {
        //         piece_index -= pieces.len() as isize;
        //     }
        //     while piece_index < 0 {
        //         piece_index += pieces.len() as isize;
        //     }
        //     curr_piece = piece_shapes.get(&pieces[piece_index as usize]).unwrap().clone();
        //     curr_piece.position = (4, (matrix.height - matrix.playable_height - 2) as isize);
        // }

        if is_key_pressed(controls.move_left) {
            game_state.move_left();
        }
        if is_key_pressed(controls.move_right) {
            game_state.move_right();
        }
        if is_key_pressed(controls.soft_drop) {
            game_state.move_down();
        }
        if is_key_pressed(controls.hold) {
            game_state.hold();
        }
        if is_key_pressed(controls.rotate_ccw) {
            game_state.rotate(Rotation::CCW);
        }
        if is_key_pressed(controls.rotate_cw) {
            game_state.rotate(Rotation::CW);
        }
        if is_key_pressed(controls.rotate_ccw_120) {
            game_state.rotate(Rotation::CCW120);
        }
        if is_key_pressed(controls.rotate_cw_120) {
            game_state.rotate(Rotation::CW120);
        }
        if is_key_pressed(controls.rotate_180) {
            game_state.rotate(Rotation::Center);
        }
        if is_key_pressed(controls.hard_drop) {
            game_state.hard_drop();
        }
        if is_key_pressed(controls.reset) {
            game_state.reset();
        }
        
        // draw flat-right
        clear_background(background);

        draw_text(format!("{}", get_fps()).as_str(), 20f32, 20f32, 20f32, WHITE);

        // if matrix.contains((mouse_q, mouse_r)) {
        //     let mouse_idx = (mouse_q as usize, mouse_r as usize);
        //     draw_text(format!("[{}, {}]", mouse_idx.0, mouse_idx.1).as_str(), 20f32, 20f32, 20f32, WHITE);
        // }

        for r in 0..game_state.matrix.playable_height {
            for q in 0..game_state.matrix.width {
                let (x, y) = flat_hex_to_pixel((q as isize, r as isize), hex_size);
                draw_hexagon(
                    offset_x + x + (border_size/2f32) + tri_width,
                    offset_y + y + (border_size/2f32) + tri_height,
                    hex_size + border_size,
                    0f32,
                    false,
                    BLANK,
                    border,
                );
                draw_hexagon(
                    offset_x + x + (border_size/2f32) + tri_width,
                    offset_y + y + (border_size/2f32) + tri_height,
                    hex_size - border_size,
                    0f32,
                    false,
                    BLANK,
                    match piece_colors.get(&game_state.matrix.get((q, r + game_state.matrix.playable_height))) {
                        Some(color) => {*color}
                        _ => {BLACK}
                    },
                );
            }
        }
        for (q, r) in game_state.curr_piece.hexagons {
            let (q, r) = (q + game_state.curr_piece.position.0, r + game_state.curr_piece.position.1 - game_state.matrix.playable_height as isize);
            let (x, y) = flat_hex_to_pixel((q, r), hex_size);
            draw_hexagon(
                offset_x + x + (border_size/2f32) + tri_width,
                offset_y + y + (border_size/2f32) + tri_height,
                hex_size - border_size,
                0f32,
                false,
                BLANK,
                match piece_colors.get(&game_state.curr_piece.name) {
                    Some(color) => {*color}
                    _ => {BLACK}
                },
            );
        }
        let ghost = game_state.matrix.ghost(&game_state.curr_piece);
        for (q, r) in ghost.hexagons {
            let (q, r) = (q + ghost.position.0, r + ghost.position.1 - game_state.matrix.playable_height as isize);
            let (x, y) = flat_hex_to_pixel((q, r), hex_size);
            draw_hexagon(
                offset_x + x + (border_size/2f32) + tri_width,
                offset_y + y + (border_size/2f32) + tri_height,
                hex_size - border_size,
                0f32,
                false,
                BLANK,
                match piece_colors.get(&ghost.name) {
                    Some(color) => {
                        Color::new(color.r, color.g, color.b, 0.5)
                    }
                    _ => {BLACK}
                },
            );
        }
        for (q, r) in game_state.hold_piece.hexagons {
            let (hold_q, hold_r) = (-4, 5);

            let (q, r) = (q + hold_q, r + hold_r);
            let (x, y) = flat_hex_to_pixel((q, r), hex_size);
            draw_hexagon(
                offset_x + x + (border_size/2f32) + tri_width,
                offset_y + y + (border_size/2f32) + tri_height,
                hex_size - border_size,
                0f32,
                false,
                BLANK,
                match piece_colors.get(&game_state.hold_piece.name) {
                    Some(color) => {*color}
                    _ => {BLACK}
                },
            );
        }
        for queue_index in 0..queue_view {
            let queue_piece_name = game_state.queue.queue.get(queue_index).unwrap();
            let queue_piece = game_state.piece_gen.translate(queue_piece_name);
            for (q, r) in queue_piece.hexagons {
                let (queue_q, queue_r) = (game_state.matrix.width as isize + 4, (-3 + (queue_index as isize * 5)));
                let (q, r) = (q + queue_q, r + queue_r);
                let (x, y) = flat_hex_to_pixel((q, r), hex_size);
                draw_hexagon(
                    offset_x + x + (border_size/2f32) + tri_width,
                    offset_y + y + (border_size/2f32) + tri_height,
                    hex_size - border_size,
                    0f32,
                    false,
                    BLANK,
                    match piece_colors.get(&queue_piece.name) {
                        Some(color) => {*color}
                        _ => {BLACK}
                    },
                );
            }
        }
        

        next_frame().await
    }
}
