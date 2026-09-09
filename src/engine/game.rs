use crate::engine::consts::{BLOCK_SIZE, COLS, ROWS};
use macroquad::color::Color;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Cell {
    value: i32,
    color: Color,
}

enum Sideways {
    Left,
    Right,
}

pub struct Game {
    board: Vec<Cell>,
    board_color: Color,
    moving_index: Vec<i32>,
}

const NUM_OF_CELLS: i32 = COLS * ROWS;
const ORIGIN_INDEX: i32 = (COLS / 2) - 2;

impl Game {
    pub fn new() -> Self {
        let board = (0..NUM_OF_CELLS)
            .map(|_| Cell {
                value: 0,
                color: BROWN,
            })
            .collect();
        Self {
            board,
            board_color: GOLD,
            moving_index: vec![],
        }
    }

    pub async fn start(&mut self) {
        let mut timer = 0.;
        loop {
            clear_background(WHITE);

            if self.moving_index.len() == 0 {
                // If we have nothing to move, create something to move
                self.spawn_block();
            }

            self.render_board();
            let mut sidways_direction: Option<Sideways> = None;
            if is_key_pressed(KeyCode::Right) {
                sidways_direction = Some(Sideways::Right);
            }
            if is_key_pressed(KeyCode::Left) {
                sidways_direction = Some(Sideways::Left);
            }

            match sidways_direction {
                Some(direction) => {
                    // some shit
                    if self.can_blocks_move_sideways(&direction) {
                        self.move_block_sideways(&direction);
                    }
                }
                None => {}
            };

            if timer >= 0.4 {
                if self.can_blocks_move_down() {
                    self.move_blocks_down();
                } else {
                    println!("cannot move down");
                    self.moving_index = vec![];
                }

                timer = 0.;
            }

            timer += get_frame_time();
            next_frame().await;
        }
    }

    pub fn render_board(&self) {
        draw_rectangle(
            0.,
            0.,
            BLOCK_SIZE * COLS as f32,
            BLOCK_SIZE * ROWS as f32,
            BROWN,
        );
        for (i, cell) in self.board.iter().enumerate() {
            let x = (i as i32 % COLS) as f32;
            let y = (i as i32 / COLS) as f32;
            draw_rectangle_lines(
                x * BLOCK_SIZE,
                y * BLOCK_SIZE,
                BLOCK_SIZE,
                BLOCK_SIZE,
                2.,
                self.board_color,
            );

            if cell.value == 1 {
                draw_rectangle(
                    x * BLOCK_SIZE,
                    y * BLOCK_SIZE,
                    BLOCK_SIZE,
                    BLOCK_SIZE,
                    cell.color,
                );
            }
        }
    }

    fn spawn_block(&mut self) {
        // Track the moving piece
        self.moving_index = vec![ORIGIN_INDEX];

        // Update the board to use new moving piece
        for val in &self.moving_index {
            self.board[*val as usize].value = 1;
            self.board[*val as usize].color = RED;
        }
    }

    fn move_blocks_down(&mut self) {
        let mut new_indexes: Vec<i32> = vec![];
        for i in (0..self.moving_index.len()).rev() {
            let down_target_index = (self.moving_index[i] + COLS) as usize;
            self.board[(self.moving_index[i]) as usize].value = 0;
            self.board[(self.moving_index[i]) as usize].color = BROWN;

            self.board[down_target_index].value = 1;
            self.board[down_target_index].color = RED;

            new_indexes.push(down_target_index as i32);
        }

        self.moving_index = new_indexes;
    }

    fn move_block_sideways(&mut self, direction: &Sideways) {
        let mut new_indexes: Vec<i32> = vec![];
        let dx: i32 = match direction {
            Sideways::Left => -1,
            Sideways::Right => 1,
        };

        for i in (0..self.moving_index.len()).rev() {
            let side_target_index = (self.moving_index[i] as i32 + dx) as usize;
            self.board[(self.moving_index[i]) as usize].value = 0;
            self.board[(self.moving_index[i]) as usize].color = BROWN;

            self.board[side_target_index].value = 1;
            self.board[side_target_index].color = RED;

            new_indexes.push(side_target_index as i32);
        }

        self.moving_index = new_indexes;
    }

    fn can_blocks_move_down(&self) -> bool {
        for i in &self.moving_index {
            let down_target_index = (i + COLS) as usize;
            if down_target_index >= self.board.len() {
                // Cannot move outside the board
                return false;
            }

            if self.board[*i as usize].value != 1  || // An empty cell  
                self.board[down_target_index].value == 1
            {
                return false;
            }
        }

        true
    }

    fn can_blocks_move_sideways(&mut self, direction: &Sideways) -> bool {
        let dx = match direction {
            Sideways::Left => -1,
            Sideways::Right => 1,
        };

        for i in &self.moving_index {
            let side_target_index = (*i + dx) as usize;
            if side_target_index >= self.board.len() || side_target_index < 0 {
                // Cannot move outside the board
                return false;
            }

            match direction {
                Sideways::Left => {
                    if *i % COLS == 0 {
                        return false;
                    }
                }
                Sideways::Right => {
                    if side_target_index as i32 % COLS == 0 {
                        return false;
                    }
                }
            }

            if self.board[*i as usize].value != 1  || // An empty cell  
                self.board[side_target_index].value == 1
            {
                return false;
            }
        }
        true
    }
}
