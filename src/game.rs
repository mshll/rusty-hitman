use crate::*;
use macroquad_particles::*;
use std::rc::Rc;
use GameState::*;

pub enum GameState {
    Menu,
    Playing,
    LevelTransition,
    GameOver,
    Paused,
}

const SCORE_BASE: f32 = 100.0;
pub const SPAWN_DELAY: f32 = 0.2;
pub const LEVEL_TIME: f32 = 10.0;
pub const TRANSITION_DELAY: f32 = 0.5;

pub struct Game {
    /// The assets bundle.
    pub assets: Rc<asset_bundle::AssetBundle>,
    /// The level struct.
    pub level: level::Level,
    /// The game state.
    pub game_state: GameState,
    /// The score, [level number, total score]
    pub score: [f32; 2],
    /// The game over flag.
    pub game_over: bool,
    /// The game renderer.
    pub renderer: renderer::Renderer,
    // Shooting particle effect
    pub bullet_fx: Emitter,
    /// The level transition timer.
    pub transition_timer: f32,
}

impl Game {
    /// Initializes a game struct.
    pub async fn init() -> Game {
        let assets = Rc::new(asset_bundle::AssetBundle::load().await.unwrap()); // Load game assets
        let level = level::Level::init(&assets);

        // Shooting particle effect.
        let bullet_fx = Emitter::new(EmitterConfig {
            emission_shape: EmissionShape::Sphere { radius: 25.0 },
            one_shot: true,
            lifetime: 0.2,
            explosiveness: 0.6,
            amount: 100,
            shape: ParticleShape::Circle { subdivisions: 10 },
            emitting: false,
            initial_direction: vec2(0.0, -1.0),
            initial_direction_spread: 6.0,
            initial_velocity: 310.0,
            linear_accel: -7.5,
            size: 15.0,
            size_curve: Some(Curve {
                points: vec![(0.005, 0.25), (0.255, 0.75), (1.0, 1.0)],
                interpolation: Interpolation::Linear,
                resolution: 30,
            }),
            blend_mode: BlendMode::Alpha,
            colors_curve: ColorCurve {
                start: Color::new(1.0, 1.0, 1.0, 1.0),
                mid: Color::new(1.0, 0.9569, 0.3961, 1.0),
                end: Color::new(0.5, 0.5, 0.5, 1.0),
            },
            gravity: vec2(0.0, 500.0),
            post_processing: Some(PostProcessing),
            ..Default::default()
        });

        let mut game = Game {
            assets,
            level,
            game_state: Menu,
            score: [0.0, 0.0],
            game_over: false,
            renderer: renderer::Renderer::init(GAME_WIDTH, GAME_HEIGHT),
            bullet_fx,
            transition_timer: TRANSITION_DELAY,
        };

        game.set_menu();
        game
    }

    /// Increments the score.
    pub fn add_score(&mut self) {
        let level_bonus = (SCORE_BASE / 10.0) * self.score[0];
        let time_bonus = (SCORE_BASE + level_bonus) * (self.level.timer / LEVEL_TIME);
        self.score[1] += SCORE_BASE + level_bonus + time_bonus;
        self.score[0] += 1.0;

        println!("Score: {:.0}, {:.0}", self.score[0], self.score[1]);
    }

    /// Draws a crosshair cursor at the mouse position.
    fn draw_cursor(&self, width: f32, height: f32) {
        let (mouse_x, mouse_y) = mouse_position();
        // Draw the custom cursor at the mouse position
        draw_texture_ex(
            self.assets.crosshair,
            mouse_x - width / 2.0,
            mouse_y - height / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(width, height)),
                ..Default::default()
            },
        );
    }

    /// Updates the game based on the game state.
    pub async fn update(&mut self) {
        self.renderer.set();
        clear_background(BG_PURPLE);

        match self.game_state {
            Menu => {
                self.menu();

                if is_key_pressed(KeyCode::Enter) {
                    self.set_level();
                }
            }

            Playing => {
                self.playing();

                if is_key_pressed(KeyCode::Escape) {
                    self.game_state = Paused;
                }
            }

            LevelTransition => {
                self.level_transition();
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

            Paused => {
                self.paused();

                if is_key_pressed(KeyCode::Escape) {
                    self.set_menu();
                } else if is_key_pressed(KeyCode::Enter) {
                    self.game_state = Playing;
                    self.level.timer_on = true;
                }
            }
        }

        self.renderer.draw();
        self.draw_cursor(96.0, 96.0);
    }

    /// Checks if the mouse clicked on a character.
    ///
    /// Returns `Some(true)` if the target character was clicked.
    /// Returns `Some(false)` if a non-target character was clicked.
    /// Returns `None` if no character was clicked.
    pub fn check_target_click(&mut self) -> Option<bool> {
        if is_mouse_button_pressed(MouseButton::Left) && self.level.timer_on {
            let (mouse_x, mouse_y) = self.renderer.mouse_position();
            println!("Mouse clicked at ({}, {})", mouse_x, mouse_y);

            // Trigger bullet particle effect
            self.bullet_fx.config.emitting = true;

            // Check if mouse clicked on a character
            for character in self.level.crowd.iter_mut() {
                if mouse_x >= character.x
                    && mouse_x <= character.x + CHAR_WIDTH
                    && mouse_y >= character.y
                    && mouse_y <= character.y + CHAR_HEIGHT
                    && character.spawned
                {
                    character.dead = true;

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
}
