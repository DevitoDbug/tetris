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
    block_variations: [[i32; 4]; 7],
}

const NUM_OF_CELLS: i32 = COLS * ROWS;
const ORIGIN_INDEX: i32 = (COLS / 2) - 2;

impl Game {
    pub fn new() -> Self {
        let variations = Self::gen_blocks();

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
            block_variations: variations,
        }
    }

    pub async fn start(&mut self) {
        let mut down_movement_timer = 0.;
        let mut sideways_movement_timer = 0.;
        let mut new_block_spawn_timer = 0.5;
        loop {
            clear_background(WHITE);

            if self.moving_index.len() == 0 && new_block_spawn_timer >= 0.5 {
                // If we have nothing to move, create something to move
                self.spawn_block();
            }

            self.render_board();
            let mut sideways_direction: Option<Sideways> = None;
            if is_key_down(KeyCode::Right) {
                sideways_direction = Some(Sideways::Right);
            }
            if is_key_down(KeyCode::Left) {
                sideways_direction = Some(Sideways::Left);
            }
            if is_key_down(KeyCode::Down) {
                if self.can_blocks_move_down() {
                    self.move_blocks_down();
                } else {
                    self.moving_index = vec![];
                    new_block_spawn_timer = 0.;
                }
            }

            if sideways_movement_timer >= 0.08 {
                match sideways_direction {
                    Some(direction) => {
                        if self.can_blocks_move_sideways(&direction) {
                            self.move_block_sideways(&direction);
                        }
                    }
                    None => {}
                };
                sideways_movement_timer = 0.;
            }

            if down_movement_timer >= 0.4 {
                if self.can_blocks_move_down() {
                    self.move_blocks_down();
                } else {
                    self.moving_index = vec![];
                    new_block_spawn_timer = 0.;
                }

                down_movement_timer = 0.;
            }

            down_movement_timer += get_frame_time();
            sideways_movement_timer += get_frame_time();
            new_block_spawn_timer += get_frame_time();
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

    fn gen_blocks() -> [[i32; 4]; 7] {
        [
            [
                ORIGIN_INDEX + 1,
                ORIGIN_INDEX + 2,
                ORIGIN_INDEX + 3 + COLS,
                ORIGIN_INDEX + 4 + COLS,
            ],
            [
                ORIGIN_INDEX,
                ORIGIN_INDEX + COLS,
                ORIGIN_INDEX + (2 * COLS),
                ORIGIN_INDEX + (3 * COLS),
            ],
            [
                ORIGIN_INDEX,
                ORIGIN_INDEX + 1,
                ORIGIN_INDEX + (1 + COLS),
                ORIGIN_INDEX + (2 + COLS),
            ],
            [
                ORIGIN_INDEX + 1,
                ORIGIN_INDEX + 2,
                ORIGIN_INDEX + COLS,
                ORIGIN_INDEX + (1 + COLS),
            ],
            [
                ORIGIN_INDEX,
                ORIGIN_INDEX + 1,
                ORIGIN_INDEX + (1 + COLS),
                ORIGIN_INDEX + (2 + COLS),
            ],
            [
                ORIGIN_INDEX,
                ORIGIN_INDEX + COLS,
                ORIGIN_INDEX + (2 * COLS),
                ORIGIN_INDEX + (2 * COLS) + 1,
            ],
            [
                ORIGIN_INDEX + 1,
                ORIGIN_INDEX + 1 + COLS,
                ORIGIN_INDEX + 1 + (2 * COLS),
                ORIGIN_INDEX + (2 * COLS),
            ],
        ]
    }

    fn spawn_block(&mut self) {
        // Track the moving piece
        self.moving_index = self.block_variations[rand::gen_range(0, 6)].to_vec();

        // Update the board to use new moving piece
        for val in &self.moving_index {
            self.board[*val as usize].value = 1;
            self.board[*val as usize].color = RED;
        }
    }

    fn move_blocks_down(&mut self) {
        let new_indexes: Vec<i32> = self.moving_index.iter().map(|x| x + COLS).collect();

        for i in (0..self.moving_index.len()).rev() {
            self.board[(self.moving_index[i]) as usize].value = 0;
            self.board[(self.moving_index[i]) as usize].color = BROWN;
        }

        for index in &new_indexes {
            self.board[*index as usize].value = 1;
            self.board[*index as usize].color = RED;
        }

        self.moving_index = new_indexes;
    }

    fn move_block_sideways(&mut self, direction: &Sideways) {
        let dx: i32 = match direction {
            Sideways::Left => -1,
            Sideways::Right => 1,
        };
        let new_indexes: Vec<i32> = self.moving_index.iter().map(|x| x + dx).collect();

        for i in 0..self.moving_index.len() {
            self.board[(self.moving_index[i]) as usize].value = 0;
            self.board[(self.moving_index[i]) as usize].color = BROWN;
        }

        for index in &new_indexes {
            self.board[*index as usize].value = 1;
            self.board[*index as usize].color = RED;
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

            if self.moving_index.contains(&(down_target_index as i32)) {
                continue;
            }

            if self.board[down_target_index].value == 1 {
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
            if side_target_index >= self.board.len() {
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

            if self.moving_index.contains(&(side_target_index as i32)) {
                continue;
            }

            if self.board[side_target_index].value == 1 {
                return false;
            }
        }

        true
    }
}
