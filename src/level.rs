//! Level struct and methods

use crate::asset_bundle::*;
use crate::character::Character;
use crate::*;
use macroquad::audio::*;
use std::rc::Rc;

const BAR_BG_WIDTH: f32 = GROUND_WIDTH;
const BAR_WIDTH: f32 = GROUND_WIDTH - 8.0;
const BAR_HEIGHT: f32 = 20.0;
const BAR_OFFSET: f32 = 4.0;

pub struct Level {
    /// The crowd of characters in the level.
    pub crowd: Vec<Character>,
    /// The target character's traits.
    pub target_traits: [usize; CHAR_PARTS_COUNT],
    /// The indices of the target character's unique traits.
    pub unique_traits_indices: Vec<usize>,
    /// The assets bundle.
    pub assets: Rc<AssetBundle>,
    /// The color to draw the hints in.
    pub hints_color: Color,
    /// The timer of the level.
    pub timer: f32,
    /// Whether the timer should be running.
    pub timer_on: bool,
    /// The delay between spawning characters.
    pub spawn_timer: f32,
    /// Iterator of the crowd used for spawning characters with the delay.
    crowd_iter: usize,
}

impl Level {
    /// Initializes a level struct.
    pub fn init(assets: &Rc<AssetBundle>) -> Level {
        Level {
            crowd: Vec::new(),
            unique_traits_indices: Vec::new(),
            target_traits: [0; CHAR_PARTS_COUNT],
            assets: Rc::clone(assets),
            hints_color: rand_color(),
            timer: LEVEL_TIME,
            timer_on: false,
            spawn_timer: SPAWN_DELAY,
            crowd_iter: 0,
        }
    }

    /// Draws the level.
    ///
    /// Returns `true` if the timer is up. Returns `false` otherwise.
    pub fn draw(&mut self, score: [f32; 2]) -> bool {
        // Draw the ground
        self.draw_ground();

        // Draw level number
        let text_size = measure_text("TEXT", Some(self.assets.font), 32, 1.0);
        draw_text_ex(
            &format!("{:.0}", score[0]),
            110.0,
            70.0,
            TextParams {
                font: self.assets.font,
                font_size: 32,
                color: WHITE,
                ..Default::default()
            },
        );
        draw_texture_ex(
            self.assets.skull,
            60.0,
            70.0 - text_size.height * 1.25,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(text_size.height * 1.5, text_size.height * 1.5)),
                ..Default::default()
            },
        );

        // Draw the total score
        draw_text_ex(
            &format!("{:.0}", score[1]),
            65.0,
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
        // self.draw_target_outline();

        // Update and draw the timer
        self.draw_progress_bar()
    }

    /// Generates a crowd of `num` random characters between the given coordinates.
    /// The first character in the crowd is the target.
    pub fn gen_crowd(&mut self, num: usize, x_min: f32, x_max: f32, y_min: f32, y_max: f32) {
        let mut traits_range: Vec<usize> = (0..CHAR_PARTS_COUNT).collect();
        traits_range.shuffle(); // Shuffle the traits range
        self.unique_traits_indices = traits_range[0..3].to_vec(); // Pick the first 3 traits as the unique traits
        self.unique_traits_indices.sort(); // Sort the unique traits indices

        self.crowd = Vec::new(); // Clear the crowd
        self.crowd_iter = 0; // Reset the crowd iterator
        self.timer_on = false;

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
                        self.assets.blood, // blood texture (for when the character is killed)
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
            GAME_WIDTH - GROUND_WIDTH - 50.0,
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
        self.spawn_timer -= get_frame_time();

        // Spawn a new character every `SPAWN_DELAY` seconds
        if self.spawn_timer <= 0.0 && self.crowd_iter < self.crowd.len() {
            self.crowd[self.crowd_iter].spawned = true;
            self.spawn_timer = SPAWN_DELAY;
            self.crowd_iter += 1;

            play_sound_once(self.assets.spawn_sound);

            // Start the timer when the last character is spawned
            if self.crowd_iter >= self.crowd.len() {
                self.timer_on = true;
            }
        }

        // Draw the crowd
        for character in self.crowd.iter_mut() {
            character.draw(true);
        }
    }

    /// Draws the hints for the target character.
    pub fn draw_hints(&self) {
        let hints_text = ["Arms", "Body", "Face", "Hat", "Legs"];
        let (x, y) = (70.0, GAME_HEIGHT - GROUND_HEIGHT + 110.0);
        let size = 108.0;
        let padding = 10.0;
        let gap = 20.0;
        let mut hints_color;

        // Draw hints background
        draw_texture_ex(
            self.assets.frame_long,
            x - 20.0,
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

        // Draw hints
        for i in 0..3 {
            let mut texture = match self.unique_traits_indices[i] {
                0 => self.assets.char_arms[self.target_traits[0]],
                1 => self.assets.char_body[self.target_traits[1]],
                2 => self.assets.char_face[self.target_traits[2]],
                3 => self.assets.char_hat[self.target_traits[3]],
                4 => self.assets.char_legs[self.target_traits[4]],
                _ => panic!("Invalid trait index!"), // TODO: Remove and use Result instead?
            };

            // If the character has no hat, draw the empty texture instead.
            if texture == self.assets.char_hat[0] {
                texture = self.assets.empty;
            }

            // Don't colorize the face and hat.
            if self.unique_traits_indices[i] == 2 || self.unique_traits_indices[i] == 3 {
                hints_color = WHITE;
            } else {
                hints_color = self.hints_color;
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

    /// Updates and draws the level progress bar.
    ///
    /// Returns `true` if the timer is up. Returns `false` otherwise.
    pub fn draw_progress_bar(&mut self) -> bool {
        let bar_color = if self.timer < (LEVEL_TIME / 3.0) {
            COLOR_RED
        } else if self.timer < (LEVEL_TIME / 3.0 * 2.0) {
            COLOR_YELLOW
        } else {
            COLOR_GREEN
        };

        // Draw progress bar background
        let bar_x = GAME_WIDTH - GROUND_WIDTH - 50.0;
        let bar_y = 80.0;
        draw_texture_ex(
            self.assets.bar[1],
            bar_x,
            bar_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(BAR_BG_WIDTH, BAR_HEIGHT)),
                ..Default::default()
            },
        );

        // Draw progress bar
        let progress = self.timer / LEVEL_TIME;
        draw_texture_ex(
            self.assets.bar[0],
            bar_x + BAR_OFFSET,
            bar_y,
            bar_color,
            DrawTextureParams {
                dest_size: Some(vec2(BAR_WIDTH * progress, BAR_HEIGHT)),
                ..Default::default()
            },
        );

        // Update timer and check if it's up
        if !self.timer_on {
            return false;
        } else if self.timer > 0.0 {
            self.timer -= get_frame_time();
        } else {
            return true;
        }
        false
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
