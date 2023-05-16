//! Character struct that represents a character in the crowd.

use crate::*;
use macroquad_particles::*;

/// Character struct that represents a character in the crowd.
pub struct Character {
    pub x: f32,
    pub y: f32,
    pub is_target: bool,
    pub textures: [Texture2D; 5],
    pub color: Color,
    pub spawned: bool,
    pub smoke_fx: Emitter,
}

impl Character {
    /// Creates a new `Character` with the given position and textures.
    pub fn init(x: f32, y: f32, textures: [Texture2D; 5]) -> Character {
        // Smoke particle effect when spawning.
        let smoke_fx = Emitter::new(EmitterConfig {
            emission_shape: EmissionShape::Sphere { radius: 50.0 },
            one_shot: true,
            lifetime: 0.5,
            explosiveness: 1.0,
            amount: 150,
            shape: ParticleShape::Circle { subdivisions: 10 },
            emitting: true,
            initial_direction: vec2(0.0, -1.0),
            initial_direction_spread: 6.0,
            initial_velocity: 310.0,
            initial_velocity_randomness: 0.6,
            linear_accel: -7.5,
            size: 10.0,
            size_randomness: 3.0,
            size_curve: Some(Curve {
                points: vec![(0.005, 1.48), (0.255, 1.08), (1.0, 0.12)],
                interpolation: Interpolation::Linear,
                resolution: 30,
            }),
            blend_mode: BlendMode::Additive,
            colors_curve: ColorCurve {
                start: Color::new(1.0, 1.0, 1.0, 1.0),
                mid: Color::new(0.8, 0.8, 0.8, 0.5),
                end: Color::new(0.5, 0.5, 0.5, 0.5),
            },
            gravity: vec2(0.0, -500.0),
            post_processing: Some(PostProcessing),
            ..Default::default()
        });

        Character {
            x,
            y,
            textures,
            is_target: false,
            color: rand_color(),
            spawned: false,
            smoke_fx,
        }
    }

    /// Draws all parts of the character.
    ///
    /// If `use_smoke` is true, the character will spawn with a smoke effect.
    pub fn draw(&mut self, use_smoke: bool) {
        if !self.spawned {
            return;
        }

        for i in 0..asset_bundle::CHAR_PARTS_COUNT {
            let mut y = self.y;
            let mut color = self.color;

            if i == 2 {
                y += 6.0; // Offset the face by 6 pixels.
            }
            if i == 2 || i == 3 {
                color = WHITE; // Don't colorize the face or hat.
            }

            draw_texture_ex(
                self.textures[i],
                self.x,
                y,
                color,
                DrawTextureParams {
                    dest_size: Some(vec2(CHAR_WIDTH, CHAR_HEIGHT)),
                    ..Default::default()
                },
            );
        }

        if use_smoke {
            self.smoke_fx
                .draw(vec2(self.x + CHAR_WIDTH / 2.0, self.y + CHAR_HEIGHT - 30.0));
        }
    }
}
