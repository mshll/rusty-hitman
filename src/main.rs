use macroquad::prelude::*;
use macroquad::rand::{gen_range, srand};

const GAME_WIDTH: i32 = 1280;
const GAME_HEIGHT: i32 = 720;
const CHARACTER_WIDTH: f32 = 30.0;
const CHARACTER_HEIGHT: f32 = 30.0;

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
    target: bool,
    eyes: u8,
    mouth: u8,
    body: u8,
    ears: u8,
}

impl Character {
    fn init(x: f32, y: f32) -> Character {
        Character {
            x,
            y,
            target: false,
            eyes: 0,
            mouth: 0,
            body: 0,
            ears: 0,
        }
    }

    fn draw(&self) {
        if self.target {
            draw_rectangle(self.x, self.y, CHARACTER_WIDTH, CHARACTER_HEIGHT, RED);
        } else {
            draw_rectangle(self.x, self.y, CHARACTER_WIDTH, CHARACTER_HEIGHT, GREEN);
        }
    }
}

/// Generates a crowd of `num` characters with unique positions.
/// The first character in the crowd is the target.
///
/// Returns a vector of characters.
///
/// TODO: Generate random character features with unique features for the target.
fn gen_crowd(num: usize) -> Vec<Character> {
    let mut crowd: Vec<Character> = Vec::new();

    // Generate `num` characters with unique positions
    for _ in 0..num {
        let mut pos_unique = false;
        let mut x = 0.0;
        let mut y = 0.0;

        while !pos_unique {
            x = gen_range(
                screen_width() / 2.5,
                screen_width() / 2.5 + 512.0 - CHARACTER_WIDTH,
            );
            y = gen_range(
                screen_height() / 2.0 - 256.0,
                screen_height() / 2.0 - 256.0 + 512.0 - CHARACTER_HEIGHT,
            );

            pos_unique = true;

            // Check if the position is unique
            for character in &crowd {
                if (character.x - x).abs() < CHARACTER_HEIGHT
                    && (character.y - y).abs() < CHARACTER_WIDTH
                {
                    pos_unique = false;
                    break;
                }
            }
        }

        crowd.push(Character::init(x, y));
    }

    crowd[0].target = true;
    crowd
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    srand(macroquad::miniquad::date::now() as u64);

    let texture: Texture2D = load_texture("ground.png").await.unwrap();
    let mut crowd = gen_crowd(10);

    loop {
        clear_background(color_u8!(24, 24, 27, 255));

        // Draw the ground
        draw_texture(
            texture,
            screen_width() / 2.5,
            screen_height() / 2.0 - 256.0,
            WHITE,
        );

        // Draw the title
        draw_text(
            "RUSTY HITMAN.",
            (screen_width() / 2.0) - 100.0,
            50.0,
            30.0,
            DARKGRAY,
        );

        // Draw the crowd
        for i in crowd.iter_mut() {
            i.draw();
        }

        next_frame().await
    }
}
