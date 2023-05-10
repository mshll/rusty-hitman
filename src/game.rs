use crate::*;
use std::rc::Rc;

pub enum GameState {
    Menu,
    PrepareLevel,
    Playing,
    GameOver,
}

use GameState::*;

pub struct Game {
    pub assets: Rc<asset_bundle::AssetBundle>,
    pub level: level::Level,
    pub game_state: GameState,
    pub score: u32,
    pub renderer: renderer::Renderer,
}

impl Game {
    /// Initializes a game struct.
    pub async fn init() -> Game {
        let assets = Rc::new(asset_bundle::AssetBundle::load().await.unwrap()); // Load game assets
        let level = level::Level::init(&assets);

        Game {
            assets,
            level,
            game_state: Menu,
            score: 0,
            renderer: renderer::Renderer::init(GAME_WIDTH, GAME_HEIGHT),
        }
    }

    /// Updates the game state.
    pub fn update(&mut self) {
        // Draw the background
        draw_texture_ex(
            self.assets.bg,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Game state logic
        match self.game_state {
            Menu => {
                self.menu();

                if is_key_pressed(KeyCode::Enter) {
                    self.game_state = Playing;
                }
            }

            PrepareLevel => {
                self.prepare_level();
                self.game_state = Playing;
            }

            Playing => {
                self.playing();

                if is_key_pressed(KeyCode::Escape) {
                    self.game_state = Menu;
                }
            }

            GameOver => {
                self.game_over();

                if is_key_pressed(KeyCode::Enter) {
                    self.game_state = PrepareLevel;
                } else if is_key_pressed(KeyCode::Escape) {
                    self.game_state = Menu;
                }
            }
        }
    }

    /// Draws the game menu.
    fn menu(&self) {
        draw_text_centered(
            "Rusty Hitman",
            GAME_WIDTH / 2.0,
            GAME_HEIGHT / 2.0 - 100.0,
            self.assets.font,
            80,
            WHITE,
        );

        draw_text_centered(
            "Press Enter to start",
            GAME_WIDTH / 2.0,
            GAME_HEIGHT / 2.0,
            self.assets.font,
            32,
            WHITE,
        );
    }

    fn prepare_level(&mut self) {
        let rand_num = gen_range(3, 10);
        self.level.gen_crowd(rand_num);
    }

    /// Draws and updates the game while playing.
    fn playing(&mut self) {
        // Draw the level
        self.level.draw(self.score);

        // Check if the player clicked on the target or another character
        if let Some(target_found) = self
            .level
            .check_target_click(self.renderer.mouse_position())
        {
            if target_found {
                self.score += 1;
                self.prepare_level();
            } else {
                self.game_state = GameOver;
            }
        }
    }

    /// Draws the game over screen.
    fn game_over(&mut self) {
        self.level.draw(self.score);

        draw_rectangle(
            GAME_WIDTH / 2.0 - 300.0,
            GAME_HEIGHT / 2.0 - 150.0,
            600.0,
            300.0,
            DARK_BLUE,
        );

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

        self.score = 0;
    }
}
