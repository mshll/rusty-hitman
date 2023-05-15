//! Character struct that represents a character in the crowd.

use crate::*;

/// Character struct that represents a character in the crowd.
pub struct Character {
    pub x: f32,
    pub y: f32,
    pub is_target: bool,
    pub textures: [Texture2D; 5],
    pub color: Color,
    pub spawned: bool,
}

impl Character {
    /// Creates a new `Character` with the given position and textures.
    pub fn init(x: f32, y: f32, textures: [Texture2D; 5]) -> Character {
        Character {
            x,
            y,
            textures,
            is_target: false,
            color: rand_color(),
            spawned: false,
        }
    }

    /// Draws all parts of the character.
    pub fn draw(&mut self) {
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
    }
}
