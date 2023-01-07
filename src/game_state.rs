
use std::collections::HashMap;
use std::mem;

use crate::hex::*;

// use macroquad::prelude::Color;
use rand::rngs::ThreadRng;
use ::rand::Rng;

pub const PIECES: &str = "szdbjliotc";

#[derive(Debug, Clone)]
pub struct Piece {
    pub hexagons: [(isize, isize); 4],
    pub position: (isize, isize),
    pub name: char,
}

impl Piece {
    pub fn new(hexagons: [(isize, isize); 4], name: char) -> Self {
        Piece {
            hexagons,
            position: (0, 0),
            name,
        }
    }

    pub fn rotate(&mut self, cw: bool) {
        for hex in self.hexagons.iter_mut() {
            let (q, r) = hex;
            let (q, r, s) = axial_to_cube((*q as f32, *r as f32));
            *hex = if cw {
                (-r as isize, -s as isize)
            } else {
                (-s as isize, -q as isize)
            }
        }
    }
}
 
pub struct HexField {
    pub field: Vec<char>,
    pub width: usize,
    pub height: usize,
    pub playable_height: usize,
}

impl HexField {
    pub fn new(width: usize, height: usize, playable_height: usize) -> Self {
        let field: Vec<char> = vec![' '; width*height];
        HexField {field, width, height, playable_height}
    }

    pub fn index(&self, (q, r): (usize, usize)) -> usize {
        r * self.width + q
    }

    pub fn get(&self, (q, r): (usize, usize)) -> char {
        self.field[self.index((q, r))]
    }

    pub fn contains(&self, (q, r): (isize, isize)) -> bool {
        !(q < 0 || r < 0 || q >= self.width as isize || r >= self.height as isize)
    }

    pub fn collide(&self, piece: &Piece) -> bool {
        for hex in piece.hexagons {
            let hex = (hex.0 + piece.position.0, hex.1 + piece.position.1);
            let (q, r) = hex;
            if !self.contains((q, r)) {
                return true;
            }
            let hex = (hex.0 as usize, hex.1 as usize);
            if self.get(hex) != ' ' {
                return true;
            }
        }
        return false;
    }

    pub fn ghost(&self, piece: &Piece) -> Piece {
        let mut ghost = piece.clone();
        while !self.collide(&ghost) {
            ghost.position.1 += 1;
        }
        ghost.position.1 -= 1;
        ghost
    }

    pub fn skim(&mut self) {
        let mut new_field = vec![' '; self.width*self.height];
        let mut new_field_r_index = self.height - 1;

        for r in (0..self.height).rev() {
            let mut full = true;
            for q in 0..self.width {
                let hexo = self.get((q, r));
                new_field[self.index((q, new_field_r_index))] = hexo;
                if hexo == ' ' {
                    full = false;
                }
            }
            if !full {
                new_field_r_index -= 1;
            }
        }

        self.field = new_field;
    }
}

pub struct Queue {
    pub queue: Vec<char>,
    rng: ThreadRng,
}

impl Queue {
    pub fn new() -> Self {
        let mut queue = Queue {
            queue: Vec::with_capacity(PIECES.len()*2),
            rng: ::rand::thread_rng(),
        };
        queue.add_bag();
        queue.add_bag();
        queue
    }

    pub fn add_bag(&mut self) {
        let mut remaining_pieces = String::from(PIECES);
        while !remaining_pieces.is_empty() {
            self.queue.push(remaining_pieces.remove(self.rng.gen_range(0..remaining_pieces.len())));
        }
    }

    pub fn pop(&mut self) -> char {
        let ret = self.queue.remove(0);
        if self.queue.len() < PIECES.len() {
            self.add_bag();
        }
        ret
    }
}

pub struct PieceGen {
    piece_shapes: HashMap<char, Piece>,
    // piece_colors: HashMap<char, Color>,
}

impl PieceGen {
    pub fn default() -> Self {
        let mut piece_shapes = HashMap::new();
        piece_shapes.insert('s', Piece::new([(-1, -1), (-1, 0 ), (0 , 0 ), (0 , 1 )], 's'));
        piece_shapes.insert('z', Piece::new([(1 , -2), (1 , -1), (0 , 0 ), (0 , 1 )], 'z'));
        piece_shapes.insert('b', Piece::new([(0 , -1), (0 , 0 ), (0 , 1 ), (1 , 0 )], 'b'));
        piece_shapes.insert('d', Piece::new([(0 , -1), (0 , 0 ), (0 , 1 ), (-1, 1 )], 'd'));
        piece_shapes.insert('j', Piece::new([(0 , -2), (0 , -1), (0 , 0 ), (-1, 1 )], 'j'));
        piece_shapes.insert('l', Piece::new([(0 , -2), (0 , -1), (0 , 0 ), (1 , 0 )], 'l'));
        piece_shapes.insert('c', Piece::new([(-1, 1 ), (-1, 0 ), (0 , -1), (1 , -1)], 'c'));
        piece_shapes.insert('t', Piece::new([(0 , -1), (0 , 0 ), (-1, 1 ), (1 , 0 )], 't'));
        piece_shapes.insert('o', Piece::new([(0 , -1), (0 , 0 ), (-1, 0 ), (1 , -1)], 'o'));
        piece_shapes.insert('i', Piece::new([(0 , -2), (0, -1 ), (0 , 0 ), (0 , 1 )], 'i'));

        // let mut piece_colors = HashMap::new();
        // piece_colors.insert('s', Color::from_rgba(0  , 255, 0  , 255));
        // piece_colors.insert('z', Color::from_rgba(255, 0  , 0  , 255));
        // piece_colors.insert('b', Color::from_rgba(255, 127, 127, 255));
        // piece_colors.insert('d', Color::from_rgba(0  , 191, 0  , 255));
        // piece_colors.insert('j', Color::from_rgba(255, 127 , 0 , 255));
        // piece_colors.insert('l', Color::from_rgba(0  , 0  , 255, 255));
        // piece_colors.insert('c', Color::from_rgba(255, 0  , 127, 255));
        // piece_colors.insert('t', Color::from_rgba(255, 0  , 255, 255));
        // piece_colors.insert('o', Color::from_rgba(255, 255, 0  , 255));
        // piece_colors.insert('i', Color::from_rgba(0  , 191, 191, 255));

        PieceGen {piece_shapes}
    }

    pub fn translate(&self, piece_name: &char) -> Piece {
        self.piece_shapes.get(piece_name).unwrap().clone()
    }
}

pub enum Rotation {
    // how many 60 degree CW rotations the enum element equates to
    CW = 1,
    CCW = 5,
    CW120 = 2,
    CCW120 = 4,
    Center = 3,
}

pub struct GameState {
    pub matrix: HexField,
    pub curr_piece: Piece,
    pub hold_piece: Piece,
    pub queue: Queue,
    pub piece_gen: PieceGen,
}

impl GameState {
    pub fn new(width: usize, height: usize, playable_height: usize) -> Self {
        let queue = Queue::new();
        let curr_piece = Piece::new([(0,0); 4], ' ');
        let hold_piece = Piece::new([(0,0); 4], ' ');
        let piece_gen = PieceGen::default();
        let matrix = HexField::new(width, height, playable_height);
        let mut game_state = GameState {
            matrix,
            curr_piece,
            hold_piece,
            queue,
            piece_gen,
        };
        game_state.reset();
        game_state
    }

    pub fn reset(&mut self) {
        self.matrix.field = vec![' '; self.matrix.height*self.matrix.width];
        self.spawn_next_piece();
        self.hold_piece = Piece::new([(0, 0), (0, 0), (0, 0), (0, 0)], ' ');
    }

    pub fn set_pos_spawn(&mut self) {
        self.curr_piece.position = (4, (self.matrix.height - self.matrix.playable_height - 2) as isize);
    }

    pub fn spawn_next_piece(&mut self) {
        self.curr_piece = self.piece_gen.translate(&self.queue.pop());
        self.set_pos_spawn();
    }

    pub fn move_left(&mut self) {
        self.curr_piece.position.0 -= 1;
        if self.matrix.collide(&self.curr_piece) {
            self.curr_piece.position.1 += 1;
            if self.matrix.collide(&self.curr_piece) {
                self.curr_piece.position.0 += 1;
                self.curr_piece.position.1 -= 1;
            }
        }
    }

    pub fn move_right(&mut self) {
        self.curr_piece.position.0 += 1;
        if self.matrix.collide(&self.curr_piece) {
            self.curr_piece.position.1 -= 1;
            if self.matrix.collide(&self.curr_piece) {
                self.curr_piece.position.0 -= 1;
                self.curr_piece.position.1 += 1;
            }
        }
    }

    pub fn move_down(&mut self) {
        self.curr_piece.position.1 += 1;
        if self.matrix.collide(&self.curr_piece) {
            self.curr_piece.position.1 -= 1;
        }
    }

    pub fn hold(&mut self) {
        if self.hold_piece.name == ' ' {
            self.set_pos_spawn();
            self.hold_piece = self.curr_piece.clone();
            //hold_piece.position = (4, (matrix.height - matrix.playable_height - 2) as isize);
            
            self.spawn_next_piece();
        } else {
            self.set_pos_spawn();
            mem::swap(&mut self.hold_piece, &mut self.curr_piece);
        }
    }

    pub fn hard_drop(&mut self) {
        let ghost = self.matrix.ghost(&self.curr_piece);
        for (q, r) in ghost.hexagons {
            let (q, r) = (q + ghost.position.0, r + ghost.position.1);
            let index = self.matrix.index((q as usize, r as usize));
            self.matrix.field[index] = ghost.name;
        }
        self.spawn_next_piece();
        self.matrix.skim();
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        let cw60_rotations = rotation as u8;
        for _ in 0..cw60_rotations {
            self.curr_piece.rotate(true);
        }
        if self.matrix.collide(&self.curr_piece) {
            for _ in 0..cw60_rotations {
                self.curr_piece.rotate(false);
            }
        }
    }
}