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
        let mut traits_range: Vec<usize> = (0..CHAR_PARTS_COUNT).collect();
        traits_range.shuffle(); // Shuffle the traits range
        self.unique_traits_indices = traits_range[0..3].to_vec(); // Pick the first 3 traits as the unique traits
        self.unique_traits_indices.sort(); // Sort the unique traits indices

        self.crowd = Vec::new(); // Clear the crowd

        let x_min = GAME_WIDTH - GROUND_WIDTH - 20.0;
        let x_max = GAME_WIDTH - CHAR_WIDTH - 40.0;
        let y_min = GAME_HEIGHT - GROUND_HEIGHT - 50.0;
        let y_max = GAME_HEIGHT - CHAR_HEIGHT - 70.0;

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
                    gen_range(0, ARMS_COUNT),
                    gen_range(0, BODY_COUNT),
                    gen_range(0, FACE_COUNT),
                    gen_range(0, HAT_COUNT),
                    gen_range(0, LEGS_COUNT),
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
            GAME_WIDTH - GROUND_WIDTH - 30.0,
            GAME_HEIGHT - GROUND_HEIGHT - 30.0,
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
        let hints_text = ["Arms", "Body", "Face", "Hat", "Legs"];
        let (x, y) = (50.0, GAME_HEIGHT - GROUND_HEIGHT + 110.0);
        let size = 108.0;
        let padding = 10.0;
        let gap = 20.0;
        let mut hints_color;

        // Draw frame around hints
        draw_texture_ex(
            self.assets.frame_long,
            30.0,
            GAME_HEIGHT - GROUND_HEIGHT - 30.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(280.0, GROUND_HEIGHT)),
                ..Default::default()
            },
        );

        // Draw objective text
        let text_size = measure_text("TEXT", Some(self.assets.font), 32, 1.0);
        draw_text_ex(
            &format!("Your mission"),
            x,
            y - 80.0,
            TextParams {
                font: self.assets.font,
                font_size: 32,
                color: WHITE,
                ..Default::default()
            },
        );
        draw_text_ex(
            &format!("is to find"),
            x,
            y - 80.0 + text_size.height + 10.0,
            TextParams {
                font: self.assets.font,
                font_size: 32,
                color: WHITE,
                ..Default::default()
            },
        );
        draw_text_ex(
            &format!("who has..."),
            x,
            y - 80.0 + (text_size.height + 10.0) * 2.0,
            TextParams {
                font: self.assets.font,
                font_size: 32,
                color: WHITE,
                ..Default::default()
            },
        );

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
            } else {
                hints_color = BLUE;
            }

            // Draw frame
            draw_texture_ex(
                self.assets.frame,
                x,
                y + i as f32 * (size + padding + gap),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(size + padding, size + padding)),
                    ..Default::default()
                },
            );

            // Draw hint
            draw_texture_ex(
                texture,
                x + padding / 2.0,
                y + i as f32 * (size + padding + gap) + padding / 2.0,
                hints_color,
                DrawTextureParams {
                    dest_size: Some(vec2(size, size)),
                    ..Default::default()
                },
            );

            // Draw hint description text
            draw_text_ex(
                hints_text[self.unique_traits_indices[i]],
                x + size + padding + 20.0,
                y + i as f32 * (size + padding + gap) + (size + text_size.height) / 2.0,
                TextParams {
                    font: self.assets.font,
                    font_size: 32,
                    color: WHITE,
                    ..Default::default()
                },
            );
        }
    }

    /// Checks if the mouse clicked on a character.
    ///
    /// Returns `Some(true)` if the target character was clicked.
    /// Returns `Some(false)` if a non-target character was clicked.
    /// Returns `None` if no character was clicked.
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

        // Draw the score
        draw_text_ex(
            &format!("Score: {}", score),
            30.0,
            110.0,
            TextParams {
                font: self.assets.font,
                font_size: 32,
                color: WHITE,
                ..Default::default()
            },
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
