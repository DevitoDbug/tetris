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
    let panel = COLS * BLOCK_SIZE as i32;
    Conf {
        window_title: String::from("Tetris"),
        window_width: panel + panel * 1 / 2,
        window_height: ROWS * BLOCK_SIZE as i32,
        window_resizable: false,

        ..Default::default()
    }
}
