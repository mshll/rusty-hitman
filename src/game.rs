use crate::*;
use std::rc::Rc;

pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

use GameState::*;

const SCORE_MAX: f32 = 1000.0;

pub struct Game {
    /// The assets bundle.
    pub assets: Rc<asset_bundle::AssetBundle>,
    /// The level struct.
    pub level: level::Level,
    /// The game state.
    pub game_state: GameState,
    /// The score, [targets eliminated, time bonus]
    pub score: [f32; 2],
    /// The game renderer.
    pub renderer: renderer::Renderer,
}

impl Game {
    /// Initializes a game struct.
    pub async fn init() -> Game {
        let assets = Rc::new(asset_bundle::AssetBundle::load().await.unwrap()); // Load game assets
        let level = level::Level::init(&assets);

        let mut game = Game {
            assets,
            level,
            game_state: Menu,
            score: [0.0, 0.0],
            renderer: renderer::Renderer::init(GAME_WIDTH, GAME_HEIGHT),
        };

        game.set_menu();
        game
    }

    /// Increments the score.
    pub fn add_score(&mut self) {
        self.score[0] += 1.0;
        self.score[1] += SCORE_MAX * (self.level.timer / LEVEL_TIME);
        println!("Score: {:.0}, {:.0}", self.score[0], self.score[1]);
    }

    /// Updates the game based on the game state.
    pub fn update(&mut self) {
        match self.game_state {
            Menu => {
                self.menu();

                if is_key_pressed(KeyCode::Enter) {
                    self.set_level();
                }
            }

            Playing => {
                self.playing();
            }

            GameOver => {
                self.game_over();

                if is_key_pressed(KeyCode::Enter) {
                    self.score = [0.0, 0.0];
                    self.set_level();
                } else if is_key_pressed(KeyCode::Escape) {
                    self.set_menu();
                }
            }
        }
    }
}
