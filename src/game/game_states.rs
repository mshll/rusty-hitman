//! Game States
//!
//! Game states logic implementation.

use super::{utils::text::*, *};
use macroquad::rand::ChooseRandom;

impl Game {
    /// Sets the game state to menu.
    pub fn set_menu(&mut self) {
        self.game_state = Menu;
        self.score = [0.0, 0.0]; // Reset the score
        utils::sound::play_sound_looped(self.assets.bg_music, 1.0);

        // Load the highscore from storage if it exists
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        if let (Some(highscore_level), Some(highscore_total)) = (
            storage.get("highscore_level"),
            storage.get("highscore_total"),
        ) {
            self.highscore = [
                highscore_level.parse::<f32>().unwrap(),
                highscore_total.parse::<f32>().unwrap(),
            ];
        }

        // Generate characters for the menu background
        self.level.gen_crowd(
            200,
            0.0,
            GAME_WIDTH - CHAR_WIDTH,
            0.0,
            GAME_HEIGHT - CHAR_HEIGHT,
        );
    }

    /// Draws the game menu.
    pub fn menu(&mut self) {
        // Draw the characters in the background
        for character in self.level.crowd.iter_mut() {
            character.spawned = true;
            character.draw(false);
        }

        draw_rectangle(0.0, 0.0, GAME_WIDTH, GAME_HEIGHT, OVERLAY_PURPLE);

        draw_texture_ex(
            self.assets.logo,
            GAME_WIDTH / 2.0 - 300.0,
            GAME_HEIGHT / 2.0 - 250.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(595.0, 133.0)),
                ..Default::default()
            },
        );

        // Draw the highscore
        if self.highscore[0] > 0.0 {
            draw_text_centered(
                &format!("- HIGHSCORE: {:.0} -", self.highscore[1]),
                GAME_WIDTH / 2.0,
                GAME_HEIGHT / 2.2,
                self.assets.font,
                32,
                WHITE,
            );
        }

        draw_blinking_text(
            "Press Enter or click to start",
            GAME_WIDTH / 2.0,
            GAME_HEIGHT / 1.3,
            self.assets.font,
            32,
            WHITE,
            1.5,
        );
    }

    /// Sets the game up for playing.
    pub fn set_level(&mut self) {
        self.game_state = Playing;
        self.game_over = false;
        set_sound_volume(self.assets.bg_music, 0.25);

        // Spawn 3 characters at first and add 1 for every 5 levels (max of 10)
        let mut num_chars = 3 + (self.score[0] / 5.0) as usize;
        if num_chars > 10 {
            num_chars = 10;
        }

        // Spawn characters inside the game ground area
        let x_min = GAME_WIDTH - GROUND_WIDTH - 40.0;
        let x_max = GAME_WIDTH - CHAR_WIDTH - 50.0;
        let y_min = GAME_HEIGHT - GROUND_HEIGHT - 50.0;
        let y_max = GAME_HEIGHT - CHAR_HEIGHT - 70.0;

        // First spawn delay, starts at 1.0s and slowly decreases to 0.1s as the player progresses
        self.level.spawn_timer = 1.0 - (self.score[0] / 20.0);
        if self.level.spawn_timer < 0.1 {
            self.level.spawn_timer = 0.1;
        }

        self.level.timer = LEVEL_TIME;
        self.level.hints_color = rand_color();
        self.level.gen_crowd(num_chars, x_min, x_max, y_min, y_max);
        self.level.crowd.shuffle();
    }

    /// Draws and updates the game while playing.
    pub async fn playing(&mut self) {
        // Draw the level
        self.game_over = self.level.draw(self.score);
        self.bullet_fx.draw(self.renderer.mouse_position().into());

        // Check if the player clicked on the target or another character
        if let Some(target_found) = self.check_target_click() {
            if target_found {
                self.add_score();
            } else {
                self.game_over = true;
            }
            self.transition_level().await;
            return;
        }

        if self.game_over {
            self.transition_level().await;
        }
    }

    /// Transitions to either the next level or the game over screen.
    pub async fn transition_level(&mut self) {
        self.level.timer_on = false;

        if !self.game_over {
            crate::draw_game_screen_for!(self, 0.5, {
                self.level.draw(self.score);
                self.bullet_fx.draw(self.renderer.mouse_position().into());
            });

            self.set_level();
        } else {
            play_sound_once(self.assets.game_over_sound);
            set_sound_volume(self.assets.bg_music, 0.0);

            draw_game_screen_for!(self, 3.0, {
                self.level.draw(self.score);
                self.bullet_fx.draw(self.renderer.mouse_position().into());
                self.level.get_target().blink();
            });
            self.level.get_target().spawned = true; // Make sure the target is drawn after blinking

            self.set_game_over();
        }
    }

    /// Sets the game state to game over.
    pub fn set_game_over(&mut self) {
        self.game_state = GameOver;

        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        if let Some(highscore) = storage.get("highscore_total") {
            if self.score[1] < highscore.parse::<f32>().unwrap() {
                return; // Don't save the highscore if it's lower than the current one
            }
        }
        // Save the highscore to storage if it's higher than the current one (or if there isn't one yet)
        storage.set("highscore_level", &self.score[0].to_string());
        storage.set("highscore_total", &self.score[1].to_string());
        self.highscore = self.score;
    }

    /// Draws the game over screen.
    pub fn game_over(&mut self) {
        self.level.draw(self.score); // Keep showing the level behind the overlay

        draw_rectangle(0.0, 0.0, GAME_WIDTH, GAME_HEIGHT, OVERLAY_PURPLE);

        draw_text_centered(
            "Game Over",
            GAME_WIDTH / 2.0,
            GAME_HEIGHT / 2.0 - 120.0,
            self.assets.font,
            80,
            WHITE,
        );

        // Draw the highscore
        if self.score[1] >= self.highscore[1] && self.highscore[0] > 0.0 {
            draw_text_centered(
                "- NEW HIGHSCORE! -",
                GAME_WIDTH / 2.0,
                GAME_HEIGHT / 2.2,
                self.assets.font,
                32,
                WHITE,
            );
        } else if self.highscore[0] > 0.0 {
            draw_text_centered(
                &format!("- HIGHSCORE: {:.0} -", self.highscore[1]),
                GAME_WIDTH / 2.0,
                GAME_HEIGHT / 2.2,
                self.assets.font,
                32,
                WHITE,
            );
        }

        draw_blinking_text(
            "Press Enter or click to restart",
            GAME_WIDTH / 2.0,
            GAME_HEIGHT / 1.3,
            self.assets.font,
            32,
            WHITE,
            1.5,
        );
        draw_blinking_text(
            "Press Esc to quit",
            GAME_WIDTH / 2.0,
            GAME_HEIGHT / 1.2,
            self.assets.font,
            32,
            WHITE,
            1.5,
        );
    }

    /// Draws the game paused screen.
    pub fn paused(&mut self) {
        self.level.timer_on = false;

        draw_rectangle(0.0, 0.0, GAME_WIDTH, GAME_HEIGHT, BG_PURPLE);

        draw_text_centered(
            "Paused",
            GAME_WIDTH / 2.0,
            GAME_HEIGHT / 2.0 - 50.0,
            self.assets.font,
            80,
            WHITE,
        );

        draw_blinking_text(
            "Press Enter or click to resume",
            GAME_WIDTH / 2.0,
            GAME_HEIGHT / 2.0 + 50.0,
            self.assets.font,
            32,
            WHITE,
            1.5,
        );
        draw_blinking_text(
            "Press Esc to quit",
            GAME_WIDTH / 2.0,
            GAME_HEIGHT / 2.0 + 100.0,
            self.assets.font,
            32,
            WHITE,
            1.5,
        );
    }

    /// Draws game quit confirmation screen and handles the input.
    pub async fn confirm_quit(&mut self) {
        next_frame().await;
        draw_game_screen!(self, {
            draw_text_centered(
                "Are you sure you want to quit?",
                GAME_WIDTH / 2.0,
                GAME_HEIGHT / 2.0 - 50.0,
                self.assets.font,
                48,
                WHITE,
            );

            draw_blinking_text(
                "Press Enter or click to confirm",
                GAME_WIDTH / 2.0,
                GAME_HEIGHT / 2.0 + 50.0,
                self.assets.font,
                32,
                WHITE,
                1.5,
            );
            draw_blinking_text(
                "Press Esc to cancel",
                GAME_WIDTH / 2.0,
                GAME_HEIGHT / 2.0 + 100.0,
                self.assets.font,
                32,
                WHITE,
                1.5,
            );

            if is_key_pressed(KeyCode::Enter) || is_mouse_button_pressed(MouseButton::Left) {
                std::process::exit(0);
            } else if is_key_pressed(KeyCode::Escape) {
                break;
            }
        });
    }
}
