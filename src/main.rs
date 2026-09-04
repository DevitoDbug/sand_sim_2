use macroquad::prelude::*;

use crate::engine::{
    consts::{BLOCK_SIZE, COLS, ROWS},
    game::Game,
};

mod engine;

#[macroquad::main(config())]
async fn main() {
    let mut game = Game::new();
    game.render().await;
}

fn config() -> Conf {
    Conf {
        window_title: String::from("Sand game"),
        window_height: BLOCK_SIZE as i32 * ROWS,
        window_width: BLOCK_SIZE as i32 * COLS,
        window_resizable: false,

        ..Default::default()
    }
}
