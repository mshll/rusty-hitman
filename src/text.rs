//! Text drawing utilities.

use macroquad::prelude::*;

/// Draws text centered on the x axis.
pub fn draw_text_centered(text: &str, x: f32, y: f32, font: Font, font_size: u16, color: Color) {
    let text_width = measure_text(text, Some(font), font_size, 1.0).width;
    draw_text_ex(
        text,
        x - text_width / 2.0,
        y,
        TextParams {
            font_size,
            font,
            color,
            ..Default::default()
        },
    );
}
