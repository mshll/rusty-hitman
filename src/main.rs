//! Rusty Hitman.
//! Meshal Almutairi 2023.
//!
//! A 2D game where the player has to find and kill the target in a crowd of people before the time runs out.
//!

mod asset_bundle;
mod character;
mod colors;
mod game;
mod game_states;
mod level;
mod renderer;
mod text;
use colors::*;
use game::*;
use macroquad::rand::{gen_range, srand, ChooseRandom};
use macroquad::{color::Color, prelude::*};
use text::*;

const GAME_WIDTH: f32 = 1280.0;
const GAME_HEIGHT: f32 = 720.0;
const CHAR_WIDTH: f32 = 120.0;
const CHAR_HEIGHT: f32 = 120.0;
const GROUND_WIDTH: f32 = 867.0;
const GROUND_HEIGHT: f32 = 564.0;
const SPAWN_DELAY: f32 = 0.25;
const LEVEL_TIME: f32 = 10.0;

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

/// Draws a crosshair cursor at the mouse position.
pub fn draw_cursor(cursor_texture: Texture2D, width: f32, height: f32) {
    let (mouse_x, mouse_y) = mouse_position();
    // Draw the custom cursor at the mouse position
    draw_texture_ex(
        cursor_texture,
        mouse_x - width / 2.0,
        mouse_y - height / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(width, height)),
            ..Default::default()
        },
    );
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    srand(macroquad::miniquad::date::now() as u64);
    show_mouse(false); // Hide the mouse cursor

    let mut game = Game::init().await;

    loop {
        clear_background(BG_PURPLE);
        game.renderer.set();
        clear_background(BG_PURPLE);

        game.update();

        game.renderer.draw();
        draw_cursor(game.assets.crosshair, 128.0, 128.0);
        next_frame().await
    }
}
