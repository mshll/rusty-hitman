use macroquad::prelude::*;
use macroquad::rand::{gen_range, srand, ChooseRandom};
mod atlas;
use atlas::TextureAtlas;

const GAME_WIDTH: i32 = 1280;
const GAME_HEIGHT: i32 = 720;
const CHARACTER_WIDTH: f32 = 30.0;
const CHARACTER_HEIGHT: f32 = 30.0;

/// Game window configuration.
fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Hitman".to_owned(),
        // fullscreen: true,
        window_resizable: false,
        window_width: GAME_WIDTH,
        window_height: GAME_HEIGHT,
        ..Default::default()
    }
}

/// Character struct that represents a character in the crowd.
struct Character {
    x: f32,
    y: f32,
    body: [Texture2D; 4], // top, bottom, left, right
    target: bool,
}

impl Character {
    fn init(x: f32, y: f32, body: [Texture2D; 4]) -> Character {
        Character {
            x,
            y,
            body,
            target: false,
        }
    }

    fn draw(&mut self) {
        draw_texture(self.body[0], self.x, self.y - CHARACTER_HEIGHT, WHITE);
        draw_texture(self.body[1], self.x, self.y + CHARACTER_HEIGHT, WHITE);
        draw_texture(self.body[2], self.x - CHARACTER_WIDTH, self.y, WHITE);
        draw_texture(self.body[3], self.x + CHARACTER_WIDTH, self.y, WHITE);
        if self.target {
            draw_rectangle(self.x, self.y, CHARACTER_WIDTH, CHARACTER_HEIGHT, RED);
        } else {
            draw_rectangle(self.x, self.y, CHARACTER_WIDTH, CHARACTER_HEIGHT, WHITE);
        }
    }
}

struct Level {
    crowd: Vec<Character>,
    unique_traits: Vec<usize>,
}

/// Generates a crowd of `num` characters with unique positions.
/// The first character in the crowd is the target.
///
/// Returns a level struct that contains the crowd and the unique traits of the target.
///
/// TODO: Generate random character features with unique features for the target.
fn gen_crowd(num: usize, atlas: &TextureAtlas) -> Level {
    let traits_range: Vec<usize> = (0..4).collect();
    let mut target_traits: [usize; 4] = [0, 0, 0, 0];

    let mut level = Level {
        crowd: Vec::new(),
        unique_traits: traits_range.choose_multiple(3).cloned().collect(), // Choose 3 random unique traits from 4 total traits
    };

    // Generate `num` characters with unique positions
    for i in 0..num {
        // Generate a position for the character
        let mut pos_unique = false;
        let mut x = 0.0;
        let mut y = 0.0;
        while !pos_unique {
            pos_unique = true;
            x = gen_range(
                screen_width() / 2.5 + 20.0,
                screen_width() / 2.5 + 704.0 - 20.0 - CHARACTER_WIDTH,
            );
            y = gen_range(
                screen_height() / 2.0 - 256.0 + 10.0,
                screen_height() / 2.0 - 256.0 + 412.0 - CHARACTER_HEIGHT,
            );

            // Check if the position is unique
            for character in &level.crowd {
                if (character.x - x).abs() < CHARACTER_HEIGHT
                    && (character.y - y).abs() < CHARACTER_WIDTH
                {
                    pos_unique = false;
                    break;
                }
            }
        }

        // Generate a random character
        loop {
            let char_rand: [usize; 4] = [
                gen_range(0, 5), // top
                gen_range(0, 5), // bottom
                gen_range(0, 5), // left
                gen_range(0, 5), // right
            ];

            // Make sure the target has 3 random unique traits from the rest of the crowd
            if i > 0
                && (target_traits[level.unique_traits[0]] == char_rand[level.unique_traits[0]]
                    || target_traits[level.unique_traits[1]] == char_rand[level.unique_traits[1]]
                    || target_traits[level.unique_traits[2]] == char_rand[level.unique_traits[2]])
            {
                // if the random character has the same unique traits as the target, regenerate the character
                continue;
            } else {
                // Else, add the character to the crowd
                level.crowd.push(Character::init(
                    x,
                    y,
                    [
                        atlas.char_body[0][char_rand[0]],
                        atlas.char_body[1][char_rand[1]],
                        atlas.char_body[2][char_rand[2]],
                        atlas.char_body[3][char_rand[3]],
                    ],
                ));

                // Set the target traits
                if i == 0 {
                    target_traits = char_rand;
                }

                break;
            }
        }
    }

    level.crowd[0].target = true;
    level
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
                && mouse_x <= character.x + CHARACTER_WIDTH
                && mouse_y >= character.y
                && mouse_y <= character.y + CHARACTER_HEIGHT
            {
                // TODO: Implement game logic for clicking on a character
                character.target = !character.target;
                break;
            }
        }
    }
}

/// Draws a crosshair cursor at the mouse position.
fn draw_cursor(cursor_texture: Texture2D) {
    let (mouse_x, mouse_y) = mouse_position();
    // Draw the custom cursor at the mouse position
    draw_texture(cursor_texture, mouse_x - 36.0, mouse_y - 36.0, WHITE);
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    srand(macroquad::miniquad::date::now() as u64);
    show_mouse(false); // Hide the mouse cursor

    let atlas = TextureAtlas::init().await; // Load the texture atlas
    let mut level = gen_crowd(10, &atlas);

    loop {
        clear_background(color_u8!(24, 24, 27, 255));
        handle_input(&mut level.crowd);

        // Draw the background
        draw_texture(atlas.bg, 0.0, 0.0, WHITE);

        // Draw the ground
        draw_texture(
            atlas.ground,
            screen_width() / 2.5,
            screen_height() / 2.0 - 256.0,
            WHITE,
        );

        // Draw the title
        draw_text("Rusty Hitman", 50.0, 70.0, 60.0, LIGHTGRAY);

        // Draw the crowd
        for i in level.crowd.iter_mut() {
            i.draw();
        }

        draw_cursor(atlas.crosshair);
        next_frame().await
    }
}
