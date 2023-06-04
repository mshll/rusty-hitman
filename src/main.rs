//! Rusty Hitman.
//! Meshal Almutairi 2023.
//!
//! A 2D game where the player has to find and kill the target in a crowd of people before the time runs out.
//!

mod asset_bundle;
mod character;
mod game;
mod game_states;
mod level;
mod renderer;
mod utils;
use game::*;
use macroquad::prelude::*;
use macroquad::rand::{gen_range, srand, ChooseRandom};
use utils::colors::*;
use utils::text::*;

const GAME_WIDTH: f32 = 1280.0;
const GAME_HEIGHT: f32 = 720.0;
const CHAR_WIDTH: f32 = 120.0;
const CHAR_HEIGHT: f32 = 120.0;
const GROUND_WIDTH: f32 = 867.0;
const GROUND_HEIGHT: f32 = 564.0;

/// Game window configuration.
fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Hitman".to_owned(),
        fullscreen: true,
        window_resizable: false,
        window_width: GAME_WIDTH as i32,
        window_height: GAME_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    srand(macroquad::miniquad::date::now() as u64);
    show_mouse(false); // Hide the mouse cursor

    let mut game = Game::init().await;
    game.update().await;
}
