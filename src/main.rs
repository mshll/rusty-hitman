//! Rusty Hitman.
//! Meshal Almutairi 2023.
//!
//! A 2D game where the player has to find and kill the target in a crowd of people before the time runs out.

mod game;
use game::*;
use macroquad::window::Conf;

/// Game window configuration.
fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Hitman".to_owned(),
        fullscreen: true,
        window_width: GAME_WIDTH as i32,
        window_height: GAME_HEIGHT as i32,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::init().await;
    game.run().await;
}
