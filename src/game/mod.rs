//! Game module.
//!
//! Game logic and implementation.

mod asset_bundle;
mod game_states;
mod level;
mod renderer;
mod utils;
use asset_bundle::*;
use level::*;
use macroquad::{audio::*, prelude::*};
use macroquad_particles::*;
use std::rc::Rc;
use utils::colors::*;
use GameState::*;

pub const GAME_WIDTH: f32 = 1280.0;
pub const GAME_HEIGHT: f32 = 720.0;
const CHAR_WIDTH: f32 = 120.0;
const CHAR_HEIGHT: f32 = 120.0;
const GROUND_WIDTH: f32 = 867.0;
const GROUND_HEIGHT: f32 = 564.0;
const SCORE_BASE: f32 = 100.0;
const SPAWN_DELAY: f32 = 0.2;
const LEVEL_TIME: f32 = 10.0;
const CLICK_OFFSET: f32 = 20.0;

pub enum GameState {
    Menu,
    Playing,
    GameOver,
    Paused,
}

pub struct Game {
    /// Game assets.
    assets: Rc<asset_bundle::AssetBundle>,
    /// The level struct.
    level: level::Level,
    /// The game state.
    game_state: GameState,
    /// The score, [level number, total score]
    score: [f32; 2],
    // Highscore.
    highscore: [f32; 2],
    /// The game over flag.
    game_over: bool,
    /// The game renderer.
    renderer: renderer::Renderer,
    // Shooting particle effect
    bullet_fx: Emitter,
}

impl Game {
    /// Initializes the game.
    pub async fn init() -> Game {
        set_pc_assets_folder("assets");
        macroquad::rand::srand(macroquad::miniquad::date::now() as u64);
        show_mouse(false); // Hide the mouse cursor

        let assets = Rc::new(asset_bundle::AssetBundle::load().await.unwrap()); // Load game assets
        let level = Level::init(&assets);

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
            highscore: [-1.0, -1.0],
            game_over: false,
            renderer: renderer::Renderer::init(GAME_WIDTH, GAME_HEIGHT),
            bullet_fx,
        };

        game.set_menu();
        game
    }

    /// Starts and updates the game based on the game state.
    pub async fn run(&mut self) {
        draw_game_screen!(self, {
            match self.game_state {
                Menu => {
                    self.menu();

                    if is_key_pressed(KeyCode::Enter) {
                        self.set_level();
                        play_sound_once(self.assets.menu_in_sound);
                    } else if is_key_pressed(KeyCode::Escape) {
                        play_sound_once(self.assets.pause_sound);
                        set_sound_volume(self.assets.bg_music, 0.0);
                        self.confirm_quit().await;
                        set_sound_volume(self.assets.bg_music, 1.0);
                    }
                }

                Playing => {
                    self.playing().await;

                    if is_key_pressed(KeyCode::Escape) {
                        self.game_state = Paused;
                        play_sound_once(self.assets.pause_sound);
                        set_sound_volume(self.assets.bg_music, 0.0);
                    }
                }

                GameOver => {
                    self.game_over();

                    if is_key_pressed(KeyCode::Enter) {
                        self.score = [0.0, 0.0];
                        self.set_level();
                        play_sound_once(self.assets.menu_in_sound);
                        utils::sound::play_sound_looped(self.assets.bg_music, 0.25);
                    } else if is_key_pressed(KeyCode::Escape) {
                        self.set_menu();
                        play_sound_once(self.assets.menu_out_sound);
                    }
                }

                Paused => {
                    self.paused();

                    if is_key_pressed(KeyCode::Escape) {
                        self.set_menu();
                        play_sound_once(self.assets.menu_out_sound);
                    } else if is_key_pressed(KeyCode::Enter) {
                        self.game_state = Playing;
                        self.level.timer_on = true;
                        play_sound_once(self.assets.menu_in_sound);
                        set_sound_volume(self.assets.bg_music, 0.25);
                    }
                }
            }
        })
    }

    /// Increments the score.
    fn add_score(&mut self) {
        let level_bonus = (SCORE_BASE / 10.0) * self.score[0];
        let time_bonus = (SCORE_BASE + level_bonus) * (self.level.timer / LEVEL_TIME);
        self.score[1] += SCORE_BASE + level_bonus + time_bonus;
        self.score[0] += 1.0;
    }

    /// Draws a crosshair cursor at the mouse position.
    fn draw_cursor(&mut self, width: f32, height: f32) {
        let (mouse_x, mouse_y) = mouse_position();
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

    /// Checks if the mouse clicked on a character.
    ///
    /// Returns `Some(true)` if the target character was clicked.
    /// Returns `Some(false)` if a non-target character was clicked.
    /// Returns `None` if no character was clicked.
    fn check_target_click(&mut self) -> Option<bool> {
        if is_mouse_button_pressed(MouseButton::Left) && self.level.timer_on {
            let (mouse_x, mouse_y) = self.renderer.mouse_position();

            // Trigger bullet particle effect
            self.bullet_fx.config.emitting = true;
            play_sound_once(self.assets.shoot_sound);

            // Check if mouse clicked on a character
            for character in self.level.crowd.iter_mut() {
                if character.spawned
                    && mouse_x >= character.x
                    && mouse_x <= character.x + CHAR_WIDTH
                    && mouse_y >= character.y + CLICK_OFFSET
                    && mouse_y <= character.y + CHAR_HEIGHT
                {
                    character.dead = true;
                    return Some(character.is_target);
                }
            }
        }
        None
    }
}

// Macros
#[macro_export]
/// Macro to draw the game screen.
macro_rules! draw_game_screen {
    ($game:expr, $code:block) => {
        loop {
            clear_background(BG_PURPLE);
            $game.renderer.set();
            clear_background(BG_PURPLE);

            $code

            $game.renderer.draw();
            $game.draw_cursor(96.0, 96.0);
            next_frame().await
        }
    };
}
pub(super) use draw_game_screen;

#[macro_export]
/// Macro to draw the game screen for `seconds` seconds.
macro_rules! draw_game_screen_for {
    ($game:expr, $seconds:expr, $code:block) => {
        let mut timer = $seconds;
        draw_game_screen!($game, {
            timer -= get_frame_time();

            $code

            if timer <= 0.0 {
                break;
            }
        })
    };
}
pub(super) use draw_game_screen_for;
