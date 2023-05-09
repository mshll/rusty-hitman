//! Level struct and methods

use crate::asset_bundle::*;
use crate::character::Character;
use crate::*;
use std::rc::Rc;

pub struct Level {
    pub crowd: Vec<Character>,
    pub target_traits: [usize; CHAR_PARTS_COUNT],
    pub unique_traits_indices: Vec<usize>,
    pub assets: Rc<AssetBundle>,
}

impl Level {
    /// Initializes a level struct.
    pub fn init(assets: &Rc<AssetBundle>) -> Level {
        Level {
            crowd: Vec::new(),
            unique_traits_indices: Vec::new(),
            target_traits: [0; CHAR_PARTS_COUNT],
            assets: Rc::clone(assets),
        }
    }

    /// Generates a crowd of `num` characters with unique positions.
    /// The first character in the crowd is the target.
    pub fn gen_crowd(&mut self, num: usize) {
        self.unique_traits_indices = (0..CHAR_PARTS_COUNT)
            .collect::<Vec<usize>>()
            .choose_multiple(3)
            .copied()
            .collect(); // Choose 3 random traits from `CHAR_PARTS_COUNT` total

        self.crowd = Vec::new(); // Clear the crowd

        let x_min = GAME_WIDTH / 2.5 + 20.0;
        let x_max = GAME_WIDTH / 2.5 + GROUND_WIDTH - 20.0 - CHAR_WIDTH;
        let y_min = GAME_HEIGHT / 2.0 - GROUND_HEIGHT / 2.0 + 10.0;
        let y_max = GAME_HEIGHT / 2.0 - GROUND_HEIGHT / 2.0 + 412.0 - CHAR_HEIGHT;

        // Generate `num` characters scattered around the level.
        for i in 0..num {
            // Generate a position for the character
            let mut pos_valid = false;
            let mut x = 0.0;
            let mut y = 0.0;

            while !pos_valid {
                pos_valid = true;
                x = gen_range(x_min, x_max);
                y = gen_range(y_min, y_max);

                // Check if the position is valid (not colliding with another character)
                for character in &self.crowd {
                    if (character.x - x).abs() < CHAR_HEIGHT && (character.y - y).abs() < CHAR_WIDTH
                    {
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
                    && (self.target_traits[self.unique_traits_indices[0]]
                        == char_rand[self.unique_traits_indices[0]]
                        && self.target_traits[self.unique_traits_indices[1]]
                            == char_rand[self.unique_traits_indices[1]]
                        && self.target_traits[self.unique_traits_indices[2]]
                            == char_rand[self.unique_traits_indices[2]])
                {
                    // generate a new character
                    continue;
                }

                // Add the character to the crowd
                self.crowd.push(Character::init(
                    x,
                    y,
                    [
                        self.assets.char_arms[char_rand[0]],
                        self.assets.char_body[char_rand[1]],
                        self.assets.char_face[char_rand[2]],
                        self.assets.char_hat[char_rand[3]],
                        self.assets.char_legs[char_rand[4]],
                    ],
                ));

                // Set the target traits
                if i == 0 {
                    self.target_traits = char_rand;
                }

                break;
            }
        }

        self.crowd[0].is_target = true;
    }

    /// Draws the ground.
    pub fn draw_ground(&self) {
        draw_texture_ex(
            self.assets.ground,
            GAME_WIDTH / 2.5,
            GAME_HEIGHT / 2.0 - GROUND_HEIGHT / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(GROUND_WIDTH, GROUND_HEIGHT)),
                ..Default::default()
            },
        );
    }

    /// Draws the crowd.
    pub fn draw_crowd(&mut self) {
        for character in self.crowd.iter_mut() {
            character.draw();
        }
    }

    /// Draws the hints for the target character.
    pub fn draw_hints(&self) {
        let (x, y) = (100.0, GAME_HEIGHT / 2.0 - GROUND_HEIGHT / 2.0);
        let mut hints_color = BLUE;

        for i in 0..3 {
            let texture = match self.unique_traits_indices[i] {
                0 => self.assets.char_arms[self.target_traits[0]],
                1 => self.assets.char_body[self.target_traits[1]],
                2 => self.assets.char_face[self.target_traits[2]],
                3 => self.assets.char_hat[self.target_traits[3]],
                4 => self.assets.char_legs[self.target_traits[4]],
                _ => panic!("Invalid trait index!"), // TODO: Remove and use Result instead?
            };

            // Don't colorize the face and hat.
            if self.unique_traits_indices[i] == 2 || self.unique_traits_indices[i] == 3 {
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

    /// Checks if the mouse clicked on a character.
    pub fn check_target_click(&mut self, mouse_pos: (f32, f32)) -> Option<bool> {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_pos;
            println!("Mouse clicked at ({}, {})", mouse_x, mouse_y);

            // Check if mouse clicked on a character
            for character in self.crowd.iter_mut() {
                if mouse_x >= character.x
                    && mouse_x <= character.x + CHAR_WIDTH
                    && mouse_y >= character.y
                    && mouse_y <= character.y + CHAR_HEIGHT
                {
                    if character.is_target {
                        return Some(true);
                    } else {
                        return Some(false);
                    }
                }
            }
        }
        None
    }

    pub fn draw(&mut self, score: u32) {
        // Draw the ground
        self.draw_ground();

        // Draw the title
        draw_text_centered(
            "Rusty Hitman",
            GAME_WIDTH / 2.0,
            50.0,
            self.assets.font,
            80,
            WHITE,
        );

        // Draw the score
        draw_text_centered(
            &format!("Score: {}", score),
            GAME_WIDTH / 2.0,
            80.0,
            self.assets.font,
            30,
            WHITE,
        );
        self.draw_crowd();
        self.draw_hints();
        self.draw_target_outline();
    }

    #[allow(unused)]
    /// Draws an outline around the target character.
    pub fn draw_target_outline(&self) {
        for character in self.crowd.iter() {
            if character.is_target {
                draw_rectangle_lines(character.x, character.y, CHAR_WIDTH, CHAR_HEIGHT, 5.0, RED);
            }
        }
    }
}
