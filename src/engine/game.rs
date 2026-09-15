use crate::engine::consts::{BLOCK_SIZE, COLS, ROWS};
use ::rand::random_range as external_rand;
use macroquad::{
    color::Color,
    prelude::*,
    ui::{root_ui, widgets},
};

#[derive(Debug)]
pub struct Cell {
    value: i32,
    color: Color,
}

enum Sideways {
    Left,
    Right,
}

#[derive(Debug)]
enum GameState {
    End,
    Restart,
    Pause,
    Play,
}

pub struct Game {
    board: Vec<Cell>,
    board_color: Color,
    moving_index: Vec<i32>,
    score: i32,
    state: GameState,
}

const NUM_OF_CELLS: i32 = COLS * ROWS;
const ORIGIN_INDEX: i32 = (COLS / 2) - 2;

impl Game {
    pub fn new() -> Self {
        let board = (0..NUM_OF_CELLS)
            .map(|_| Cell {
                value: 0,
                color: BLACK,
            })
            .collect();

        Self {
            board,
            board_color: GOLD,
            moving_index: vec![],
            score: 0,
            state: GameState::Play,
        }
    }

    pub async fn start(&mut self) {
        let mut down_movement_timer = 0.;
        let mut sideways_movement_timer = 0.;
        let mut new_block_spawn_timer = 0.5;

        loop {
            clear_background(WHITE);

            match self.state {
                GameState::End => {
                    self.end_game();
                    self.render_sidebar();
                }
                GameState::Play => {
                    self.render_board();
                    self.render_sidebar();
                    self.play_game(
                        &mut down_movement_timer,
                        &mut sideways_movement_timer,
                        &mut new_block_spawn_timer,
                    );
                    down_movement_timer += get_frame_time();
                    sideways_movement_timer += get_frame_time();
                    new_block_spawn_timer += get_frame_time();

                    if self.check_is_end_game() {
                        self.state = GameState::End;
                    }
                }
                GameState::Pause => {
                    self.render_board();
                    self.render_sidebar();
                    self.pause_game();
                }

                GameState::Restart => {
                    self.board = (0..NUM_OF_CELLS)
                        .map(|_| Cell {
                            value: 0,
                            color: BLACK,
                        })
                        .collect();

                    self.moving_index = vec![];
                    self.state = GameState::Play;
                    self.score = 0;
                    self.play_game(
                        &mut down_movement_timer,
                        &mut sideways_movement_timer,
                        &mut new_block_spawn_timer,
                    );
                }
            }

            self.check_score();
            self.clean_board();
            next_frame().await;
        }
    }

    pub fn end_game(&self) {
        let width = BLOCK_SIZE * COLS as f32;
        let height = BLOCK_SIZE * ROWS as f32;
        self.render_board();
        draw_rectangle(width / 60., height / 2. - 70., width - 10., 100., WHITE);
        draw_text("Game Over", width / 50., height / 2., 80., RED);
    }

    pub fn pause_game(&self) {
        let width = BLOCK_SIZE * COLS as f32;
        let height = BLOCK_SIZE * ROWS as f32;
        self.render_board();
        draw_rectangle(width / 60., height / 2. - 70., width - 10., 100., WHITE);
        draw_text("Pause", width / 5., height / 2., 80., BLUE);
    }

    pub fn play_game(
        &mut self,
        down_movement_timer: &mut f32,
        sideways_movement_timer: &mut f32,
        new_block_spawn_timer: &mut f32,
    ) {
        if self.moving_index.len() == 0 && *new_block_spawn_timer >= 0.5 {
            // If we have nothing to move, create something to move
            self.spawn_block();
        }

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
                *new_block_spawn_timer = 0.;
            }
        }

        if *sideways_movement_timer >= 0.08 {
            match sideways_direction {
                Some(direction) => {
                    if self.can_blocks_move_sideways(&direction) {
                        self.move_block_sideways(&direction);
                    }
                }
                None => {}
            };
            *sideways_movement_timer = 0.;
        }

        if *down_movement_timer >= 0.4 {
            if self.can_blocks_move_down() {
                self.move_blocks_down();
            } else {
                self.moving_index = vec![];
                *new_block_spawn_timer = 0.;
            }

            *down_movement_timer = 0.;
        }
    }

    fn check_score(&mut self) {
        let mut total_points = 0;
        let mut row_end = self.board.len() as i32;
        let mut row_start = row_end - COLS;
        while row_start >= 0 {
            let mut row_points = 0;
            for i in row_start..row_end {
                if self.board[i as usize].value != 1 || self.moving_index.contains(&i) {
                    break;
                }
                row_points += 1;
            }
            if row_points == COLS {
                total_points += row_points;
                // no longer clearing cells here — clean_board handles that via the shift
            }
            row_end = row_start;
            row_start = row_end - COLS;
        }

        self.score += total_points;
    }

    fn clean_board(&mut self) {
        let mut row_end = self.board.len() as i32;
        let mut row_start = row_end - COLS;

        while row_start >= 0 {
            let mut cells = 0;

            for i in row_start..row_end {
                if self.board[i as usize].value != 1 || self.moving_index.contains(&(i as i32)) {
                    break;
                }

                cells += 1;
            }

            if cells == COLS {
                // Shift everything above this row down.
                for i in (COLS..row_end).rev() {
                    let target_index = (i - COLS) as usize;

                    self.board[i as usize].value = self.board[target_index].value;
                    self.board[i as usize].color = self.board[target_index].color;
                }

                // Empty the top row.
                for i in 0..COLS {
                    self.board[i as usize].value = 0;
                    self.board[i as usize].color = BLACK;
                }
            } else {
                row_end = row_start;
                row_start = row_end - COLS;
            }
        }
    }

    fn check_is_end_game(&self) -> bool {
        if self.board[ORIGIN_INDEX as usize].value == 1
            && !self.moving_index.contains(&ORIGIN_INDEX)
        {
            return true;
        }

        false
    }

    fn render_board(&self) {
        draw_rectangle(
            0.,
            0.,
            BLOCK_SIZE * COLS as f32,
            BLOCK_SIZE * ROWS as f32,
            BLACK,
        );
        for (i, cell) in self.board.iter().enumerate() {
            let x = (i as i32 % COLS) as f32;
            let y = (i as i32 / COLS) as f32;
            draw_rectangle_lines(
                x * BLOCK_SIZE,
                y * BLOCK_SIZE,
                BLOCK_SIZE,
                BLOCK_SIZE,
                1.,
                self.board_color,
            );

            if cell.value == 1 {
                draw_rectangle(
                    x * BLOCK_SIZE,
                    y * BLOCK_SIZE,
                    BLOCK_SIZE - 1.,
                    BLOCK_SIZE - 1.,
                    cell.color,
                );
            }
        }
    }

    fn render_sidebar(&mut self) {
        let side_panel_w = COLS as f32 * BLOCK_SIZE * 0.5;
        let side_panel_h = ROWS as f32 * BLOCK_SIZE;
        let side_panel_x = COLS as f32 * BLOCK_SIZE;
        let btn_height = 60.;
        let btn_width = 150.;

        draw_rectangle(side_panel_x, 0., side_panel_w, side_panel_h, BLACK);

        let ui = &mut root_ui();

        draw_rectangle(side_panel_x + 2., 0.2, btn_width, btn_height * 2., WHITE);
        draw_text(
            format!("SCORE: {}", self.score),
            side_panel_x + 2.,
            72.,
            25.,
            PURPLE,
        );

        match self.state {
            GameState::Play => {
                if widgets::Button::new("PAUSE")
                    .position(vec2(side_panel_x + 2., side_panel_h * 0.2))
                    .size(vec2(btn_width, btn_height))
                    .ui(ui)
                {
                    self.state = GameState::Pause;
                }
            }
            GameState::End => {
                if widgets::Button::new("RESTART")
                    .position(vec2(side_panel_x + 2., side_panel_h * 0.2))
                    .size(vec2(btn_width, btn_height))
                    .ui(ui)
                {
                    self.state = GameState::Restart;
                }
            }
            _ => {
                if widgets::Button::new("PLAY")
                    .position(vec2(side_panel_x + 2., side_panel_h * 0.2))
                    .size(vec2(btn_width, btn_height))
                    .ui(ui)
                {
                    self.state = GameState::Play;
                }
                if widgets::Button::new("RESTART")
                    .position(vec2(
                        side_panel_x + 2.,
                        side_panel_h * 0.2 + btn_height + 4.,
                    ))
                    .size(vec2(btn_width, btn_height))
                    .ui(ui)
                {
                    self.state = GameState::Restart;
                }
            }
        }
    }

    fn gen_blocks() -> ([i32; 4], Color) {
        let variations = [
            [
                ORIGIN_INDEX,
                ORIGIN_INDEX + 1,
                ORIGIN_INDEX + COLS,
                ORIGIN_INDEX + COLS + 1,
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
                ORIGIN_INDEX + 2,
            ],
            [
                ORIGIN_INDEX + COLS,
                ORIGIN_INDEX + COLS + 1,
                ORIGIN_INDEX + 1,
                ORIGIN_INDEX + COLS + 2,
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
        ];

        let colors = [RED, PURPLE, BLUE, DARKGREEN, BROWN, MAGENTA];

        (
            variations[external_rand(0..=variations.len() - 1)],
            colors[external_rand(0..=colors.len() - 1)],
        )
    }

    fn spawn_block(&mut self) {
        // Track the moving piece
        let (rand_block, rand_color) = Self::gen_blocks();

        // The blocks could have reached to the very top
        for block in rand_block {
            if self.board[block as usize].value == 1 {
                self.state = GameState::End;
                return;
            }
        }

        self.moving_index = rand_block.to_vec();
        // Update the board to use new moving piece
        for val in &self.moving_index {
            self.board[*val as usize].value = 1;
            self.board[*val as usize].color = rand_color;
        }
    }

    fn move_blocks_down(&mut self) {
        let new_indexes: Vec<i32> = self.moving_index.iter().map(|x| x + COLS).collect();
        let mut color: Option<Color> = None;

        for i in 0..self.moving_index.len() {
            self.board[(self.moving_index[i]) as usize].value = 0;
            color = Some(self.board[(self.moving_index[i]) as usize].color);
        }

        for index in &new_indexes {
            self.board[*index as usize].value = 1;
            match color {
                Some(color) => {
                    self.board[*index as usize].color = color;
                }
                None => {}
            }
        }

        self.moving_index = new_indexes;
    }

    fn move_block_sideways(&mut self, direction: &Sideways) {
        let dx: i32 = match direction {
            Sideways::Left => -1,
            Sideways::Right => 1,
        };
        let new_indexes: Vec<i32> = self.moving_index.iter().map(|x| x + dx).collect();
        let mut color: Option<Color> = None;

        for i in 0..self.moving_index.len() {
            self.board[(self.moving_index[i]) as usize].value = 0;
            color = Some(self.board[(self.moving_index[i]) as usize].color);
        }

        for index in &new_indexes {
            self.board[*index as usize].value = 1;
            match color {
                Some(color) => {
                    self.board[*index as usize].color = color;
                }
                None => {}
            }
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
