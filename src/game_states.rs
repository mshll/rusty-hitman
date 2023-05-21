//! # Game States
//!
//! Game states logic implementation.

use crate::*;
use GameState::*;

impl Game {
    /// Sets the game state to menu.
    pub fn set_menu(&mut self) {
        self.game_state = Menu;
        self.score = [0.0, 0.0]; // Reset the score

        // Load the highscore from storage if it exists
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        if let [Some(highscore_0), Some(highscore_1)] =
            [storage.get("highscore_0"), storage.get("highscore_1")]
        {
            self.highscore = [
                highscore_0.parse::<f32>().unwrap(),
                highscore_1.parse::<f32>().unwrap(),
            ];
        }

        // Generate 25 characters for the menu background
        self.level.gen_crowd(
            25,
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

        draw_text_centered(
            "Press Enter to start",
            GAME_WIDTH / 2.0,
            GAME_HEIGHT / 1.3,
            self.assets.font,
            32,
            WHITE,
        );
    }

    /// Sets the game up for playing.
    pub fn set_level(&mut self) {
        self.game_state = Playing;
        self.game_over = false;

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

        self.level.timer = LEVEL_TIME;
        self.level.spawn_timer = 1.0; // Delay the first spawn by 1 second
        self.level.hints_color = rand_color();
        self.level.gen_crowd(num_chars, x_min, x_max, y_min, y_max);
        self.level.crowd.shuffle();
    }

    /// Draws and updates the game while playing.
    pub fn playing(&mut self) {
        // Draw the level
        let mut game_over = self.level.draw(self.score);

        // Check if the player clicked on the target or another character
        if let Some(target_found) = self.check_target_click() {
            if target_found {
                self.add_score();
                self.game_state = LevelTransition;
            } else {
                game_over = true;
            }
        }

        // Check if the game is over (time is up)
        if game_over {
            self.game_over = true;
            self.game_state = LevelTransition;
        }

        // Show shooting particle effect (only plays when the mouse is pressed)
        self.bullet_fx.draw(self.renderer.mouse_position().into());
    }

    pub fn level_transition(&mut self) {
        if self.transition_timer > 0.0 {
            self.transition_timer -= get_frame_time();
            self.level.draw(self.score);
            self.level.timer_on = false;

            self.bullet_fx.draw(self.renderer.mouse_position().into());
            return;
        }

        self.transition_timer = TRANSITION_DELAY;
        if !self.game_over {
            self.set_level();
        } else {
            self.set_game_over();
        }
    }

    /// Sets the game state to game over.
    pub fn set_game_over(&mut self) {
        self.game_state = GameOver;

        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        if let Some(highscore) = storage.get("highscore_1") {
            if self.score[1] > highscore.parse::<f32>().unwrap() {
                storage.set("highscore_1", &self.score[1].to_string());
            }
        } else {
            storage.set("highscore_0", &self.score[0].to_string());
            storage.set("highscore_1", &self.score[1].to_string());
        }
    }

    /// Draws the game over screen.
    pub fn game_over(&mut self) {
        self.level.draw(self.score); // Keep showing the level behind the overlay

        draw_rectangle(0.0, 0.0, GAME_WIDTH, GAME_HEIGHT, OVERLAY_PURPLE);

        draw_text_centered(
            "Game Over",
            GAME_WIDTH / 2.0,
            GAME_HEIGHT / 2.0 - 50.0,
            self.assets.font,
            80,
            WHITE,
        );

        draw_text_centered(
            "Press Enter to restart",
            GAME_WIDTH / 2.0,
            GAME_HEIGHT / 2.0 + 50.0,
            self.assets.font,
            32,
            WHITE,
        );
    }

    /// Draws the game paused screen.
    pub fn paused(&mut self) {
        self.level.timer_on = false;
        self.level.draw(self.score);

        draw_rectangle(0.0, 0.0, GAME_WIDTH, GAME_HEIGHT, BG_PURPLE);

        draw_text_centered(
            "Paused",
            GAME_WIDTH / 2.0,
            GAME_HEIGHT / 2.0 - 50.0,
            self.assets.font,
            80,
            WHITE,
        );

        draw_text_centered(
            "Press Enter to resume",
            GAME_WIDTH / 2.0,
            GAME_HEIGHT / 2.0 + 50.0,
            self.assets.font,
            32,
            WHITE,
        );
    }
}
