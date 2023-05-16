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
            GAME_HEIGHT / 2.0 - 200.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(595.0, 133.0)),
                ..Default::default()
            },
        );

        draw_text_centered(
            "Press Enter to start",
            GAME_WIDTH / 2.0,
            GAME_HEIGHT / 1.5,
            self.assets.font,
            32,
            WHITE,
        );
    }

    /// Sets the game up for playing.
    pub fn set_level(&mut self) {
        self.game_state = Playing;

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
        if let Some(target_found) = self
            .level
            .check_target_click(self.renderer.mouse_position())
        {
            if target_found {
                self.add_score();
                self.set_level();
            } else {
                game_over = true;
            }
        }

        // Check if the game is over (time is up)
        if game_over {
            self.game_state = GameOver;
            self.level.timer_on = false;
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
}
