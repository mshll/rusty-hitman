//! Predefined colors and a function to generate a random color.

use macroquad::prelude::*;
use macroquad::rand::gen_range;

pub const COLOR_BLUE: Color = color_u8!(64, 184, 230, 255);
pub const COLOR_RED: Color = color_u8!(234, 74, 110, 255);
pub const COLOR_ORANGE: Color = color_u8!(237, 121, 108, 255);
pub const COLOR_GREEN: Color = color_u8!(148, 187, 116, 255);
pub const COLOR_YELLOW: Color = color_u8!(246, 230, 161, 255);
pub const BG_PURPLE: Color = color_u8!(35, 22, 44, 255);
pub const OVERLAY_PURPLE: Color = color_u8!(35, 22, 44, 200);

/// Returns a random `Color` from a list of predefined colors.
pub fn rand_color() -> Color {
    let colors = [
        COLOR_BLUE,
        COLOR_RED,
        COLOR_ORANGE,
        COLOR_GREEN,
        COLOR_YELLOW,
    ];
    let i = gen_range(0, colors.len());
    colors[i]
}
