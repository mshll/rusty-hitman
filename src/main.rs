use macroquad::rand::{gen_range, srand, ChooseRandom};
use macroquad::{color::Color, prelude::*};
mod atlas;
use atlas::*;

const GAME_WIDTH: i32 = 1280;
const GAME_HEIGHT: i32 = 720;
const CHAR_WIDTH: f32 = 72.0;
const CHAR_HEIGHT: f32 = 72.0;
const GROUND_HEIGHT: f32 = 512.0;
const GROUND_WIDTH: f32 = 704.0;

const BLUE: Color = color_u8!(44, 197, 246, 255);
const RED: Color = color_u8!(249, 59, 82, 255);
const ORANGE: Color = color_u8!(252, 104, 59, 255);
const GREEN: Color = color_u8!(52, 227, 119, 255);

/// Game window configuration.
fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Hitman".to_owned(),
        fullscreen: true,
        window_resizable: false,
        window_width: GAME_WIDTH,
        window_height: GAME_HEIGHT,
        ..Default::default()
    }
}

/// Returns a random `Color` from a list of predefined colors.
pub fn rand_color() -> Color {
    let colors = [BLUE, RED, ORANGE, GREEN];
    let i = gen_range(0, colors.len());
    colors[i]
}

/// Character struct that represents a character in the crowd.
struct Character {
    x: f32,
    y: f32,
    target: bool,
    textures: [Texture2D; 5],
    color: Color,
}

impl Character {
    fn init(x: f32, y: f32, textures: [Texture2D; 5]) -> Character {
        Character {
            x,
            y,
            textures,
            target: false,
            color: rand_color(),
        }
    }

    /// Draws all parts of the character.
    fn draw(&mut self) {
        for i in 0..CHAR_PARTS_COUNT {
            let mut y = self.y;
            let mut color = self.color;

            if i == 2 {
                y += 6.0; // Offset the face by 6 pixels.
            }
            if i == 2 || i == 3 {
                color = WHITE; // Don't colorize the face or hat.
            }

            draw_texture_ex(
                self.textures[i],
                self.x,
                y,
                color,
                DrawTextureParams {
                    dest_size: Some(vec2(CHAR_WIDTH, CHAR_HEIGHT)),
                    ..Default::default()
                },
            );
        }
    }
}

struct Level {
    crowd: Vec<Character>,
    target_traits: [usize; CHAR_PARTS_COUNT],
    unique_traits_indices: Vec<usize>,
}

/// Generates a crowd of `num` characters with unique positions.
/// The first character in the crowd is the target.
///
/// Returns a level struct that contains the crowd and the unique traits of the target.
fn gen_crowd(num: usize, atlas: &TextureAtlas) -> Level {
    let mut target_traits: [usize; CHAR_PARTS_COUNT] = [0; CHAR_PARTS_COUNT];
    let traits_range: Vec<usize> = (0..CHAR_PARTS_COUNT).collect();
    let mut level = Level {
        crowd: Vec::new(),
        unique_traits_indices: traits_range.choose_multiple(3).copied().collect(), // Choose 3 random unique traits from 4 total traits
        target_traits,
    };

    // Generate `num` characters scattered around the level.
    for i in 0..num {
        // Generate a position for the character
        let mut pos_valid = false;
        let mut x = 0.0;
        let mut y = 0.0;
        let x_min = screen_width() / 2.5 + 20.0;
        let x_max = screen_width() / 2.5 + GROUND_WIDTH - 20.0 - CHAR_WIDTH;
        let y_min = screen_height() / 2.0 - GROUND_HEIGHT / 2.0 + 10.0;
        let y_max = screen_height() / 2.0 - GROUND_HEIGHT / 2.0 + 412.0 - CHAR_HEIGHT;

        while !pos_valid {
            pos_valid = true;
            x = gen_range(x_min, x_max);
            y = gen_range(y_min, y_max);

            // Check if the position is valid (not colliding with another character)
            for character in &level.crowd {
                if (character.x - x).abs() < CHAR_HEIGHT && (character.y - y).abs() < CHAR_WIDTH {
                    pos_valid = false;
                    break;
                }
            }
        }

        // Generate a random character
        loop {
            let char_rand: [usize; CHAR_PARTS_COUNT] = [
                gen_range(0, ARMS_COUNT - 1),
                gen_range(0, BODY_COUNT - 1),
                gen_range(0, FACE_COUNT - 1),
                gen_range(0, HAT_COUNT - 1),
                gen_range(0, LEGS_COUNT - 1),
            ];

            // Make sure no other character has the exact same traits as the target unique traits
            if i > 0
                && (target_traits[level.unique_traits_indices[0]]
                    == char_rand[level.unique_traits_indices[0]]
                    && target_traits[level.unique_traits_indices[1]]
                        == char_rand[level.unique_traits_indices[1]]
                    && target_traits[level.unique_traits_indices[2]]
                        == char_rand[level.unique_traits_indices[2]])
            {
                // if the random character has the same unique traits as the target, regenerate the character
                continue;
            } else {
                // Else, add the character to the crowd
                level.crowd.push(Character::init(
                    x,
                    y,
                    [
                        atlas.char_arms[char_rand[0]],
                        atlas.char_body[char_rand[1]],
                        atlas.char_face[char_rand[2]],
                        atlas.char_hat[char_rand[3]],
                        atlas.char_legs[char_rand[4]],
                    ],
                ));

                // Set the target traits
                if i == 0 {
                    target_traits = char_rand;
                    level.target_traits = target_traits;
                }

                break;
            }
        }
    }

    level.crowd[0].target = true;
    level
}

#[allow(unused)]
/// Draws an outline around the target character.
fn draw_target_outline(crowd: &[Character]) {
    for character in crowd.iter() {
        if character.target {
            draw_rectangle_lines(character.x, character.y, CHAR_WIDTH, CHAR_HEIGHT, 5.0, RED);
        }
    }
}

/// Draws the hints for the target character.
fn draw_hints(level: &Level, atlas: &TextureAtlas) {
    let (x, y) = (100.0, screen_height() / 2.0 - GROUND_HEIGHT / 2.0);
    let mut hints_color = BLUE;

    for i in 0..3 {
        let texture = match level.unique_traits_indices[i] {
            0 => atlas.char_arms[level.target_traits[0]],
            1 => atlas.char_body[level.target_traits[1]],
            2 => atlas.char_face[level.target_traits[2]],
            3 => atlas.char_hat[level.target_traits[3]],
            4 => atlas.char_legs[level.target_traits[4]],
            _ => panic!("Invalid trait index!"), // TODO: Remove and use Result instead?
        };

        if level.unique_traits_indices[i] == 2 || level.unique_traits_indices[i] == 3 {
            hints_color = WHITE;
        }

        let padding = 120.0;
        draw_rectangle(
            x,
            y + i as f32 * padding,
            CHAR_WIDTH + padding / 4.0,
            CHAR_HEIGHT + padding / 4.0,
            DARKGRAY,
        );
        draw_texture_ex(
            texture,
            x,
            y + i as f32 * padding,
            hints_color,
            DrawTextureParams {
                dest_size: Some(vec2(
                    CHAR_WIDTH + padding / 4.0,
                    CHAR_HEIGHT + padding / 4.0,
                )),
                ..Default::default()
            },
        );
    }
}

/// Handles input from the user.
/// * Checks if the user clicked on a character.
fn handle_input(crowd: &mut [Character]) {
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
                if character.target {
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

    let atlas = TextureAtlas::load().await.unwrap(); // Load the texture atlas
    let mut level = gen_crowd(10, &atlas);

    loop {
        clear_background(WHITE);
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
            screen_width() / 2.5,
            screen_height() / 2.0 - GROUND_HEIGHT / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(GROUND_WIDTH, GROUND_HEIGHT)),
                ..Default::default()
            },
        );

        // draw_target_outline(&level.crowd);

        // Draw the title
        draw_text("Rusty Hitman", 50.0, 70.0, 60.0, LIGHTGRAY);

        // Draw the crowd
        for i in level.crowd.iter_mut() {
            i.draw();
        }

        draw_hints(&level, &atlas);

        draw_cursor(atlas.crosshair);
        next_frame().await
    }
}
