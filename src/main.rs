//! Rusty Hitman.
//! Meshal Almutairi 2023.
//!
//! A 2D game where the player has to find and kill the target in a crowd of people before the time runs out.
//!

mod atlas;
mod character;
mod colors;
mod level;
use colors::*;
use macroquad::rand::{gen_range, srand, ChooseRandom};
use macroquad::{color::Color, prelude::*};

const GAME_WIDTH: f32 = 1280.0;
const GAME_HEIGHT: f32 = 720.0;
const CHAR_WIDTH: f32 = 72.0;
const CHAR_HEIGHT: f32 = 72.0;
const GROUND_HEIGHT: f32 = 512.0;
const GROUND_WIDTH: f32 = 704.0;

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

/// Handles input from the user.
/// * Checks if the user clicked on a character.
fn handle_input(crowd: &mut [character::Character]) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        println!("Mouse clicked at ({}, {})", mouse_x, mouse_y);

        // Check if mouse clicked on a character
        for character in crowd.iter_mut() {
            if mouse_x >= character.x
                && mouse_x <= character.x + CHAR_WIDTH
                && mouse_y >= character.y
                && mouse_y <= character.y + CHAR_HEIGHT
            {
                // TODO: Implement game logic for clicking on a character
                if character.is_target {
                    character.color = BLACK;
                } else {
                    character.color = WHITE;
                }
                break;
            }
        }
    }
}

/// Draws a crosshair cursor at the mouse position.
pub fn draw_cursor(cursor_texture: Texture2D) {
    let (mouse_x, mouse_y) = mouse_position();
    // Draw the custom cursor at the mouse position
    draw_texture(
        cursor_texture,
        mouse_x - cursor_texture.width() / 2.0,
        mouse_y - cursor_texture.height() / 2.0,
        WHITE,
    );
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    srand(macroquad::miniquad::date::now() as u64);
    show_mouse(false); // Hide the mouse cursor

    let atlas = atlas::TextureAtlas::load().await.unwrap(); // Load the texture atlas
    let mut level = level::Level::init(&atlas);

    level.gen_crowd(10);

    loop {
        clear_background(color_u8!(23, 22, 41, 255));
        handle_input(&mut level.crowd);

        // Draw the background
        draw_texture_ex(
            atlas.bg,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Draw the ground
        draw_texture_ex(
            atlas.ground,
            GAME_WIDTH / 2.5,
            GAME_HEIGHT / 2.0 - GROUND_HEIGHT / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(GROUND_WIDTH, GROUND_HEIGHT)),
                ..Default::default()
            },
        );

        level.draw_target_outline();

        // Draw the title
        draw_text("Rusty Hitman", 50.0, 70.0, 60.0, LIGHTGRAY);

        level.draw_crowd();

        level.draw_hints();
        draw_cursor(atlas.crosshair);
        next_frame().await
    }
}
