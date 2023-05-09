//! Predefined colors and a function to generate a random color.

use macroquad::prelude::*;
use macroquad::rand::gen_range;

pub const BLUE: Color = color_u8!(44, 197, 246, 255);
pub const RED: Color = color_u8!(249, 59, 82, 255);
pub const ORANGE: Color = color_u8!(252, 104, 59, 255);
pub const GREEN: Color = color_u8!(52, 227, 119, 255);
pub const DARK_BLUE: Color = color_u8!(23, 22, 41, 255);

/// Returns a random `Color` from a list of predefined colors.
pub fn rand_color() -> Color {
    let colors = [BLUE, RED, ORANGE, GREEN];
    let i = gen_range(0, colors.len());
    colors[i]
}
