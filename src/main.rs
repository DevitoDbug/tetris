use crate::engine::{
    consts::{BLOCK_SIZE, COLS, ROWS},
    game::Game,
};
use macroquad::prelude::*;

mod engine;

#[macroquad::main(config())]
async fn main() {
    let mut game = Game::new();
    game.start().await;
}

fn config() -> Conf {
    let right_panel = 10 * BLOCK_SIZE as i32;
    Conf {
        window_title: String::from("Tetris"),
        window_width: (COLS * BLOCK_SIZE as i32) + right_panel,
        window_height: ROWS * BLOCK_SIZE as i32,
        window_resizable: false,

        ..Default::default()
    }
}
