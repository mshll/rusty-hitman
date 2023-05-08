//! Renderer
//! - Based on mq_render_area_2d by crumblingstatue - https://github.com/crumblingstatue/mq_render_area_2d
//!
//! 2D renderer that stretches the game to fit the screen while keeping the aspect ratio.
//! It renders the game to a render target and then draws the render target to the screen scaled with the correct aspect ratio.

use macroquad::prelude::*;

pub struct Renderer {
    render_target: RenderTarget,
    width: f32,
    height: f32,
    scale: f32,
    camera: Camera2D,
}

impl Renderer {
    /// Initialize the renderer with the given virtual width and height.
    pub fn init(width: f32, height: f32) -> Renderer {
        let render_target = render_target(width as u32, height as u32);
        let camera = Camera2D {
            render_target: Some(render_target),
            zoom: vec2(2.0 / width, 2.0 / height),
            target: vec2(width / 2.0, height / 2.0),
            ..Default::default()
        };
        render_target.texture.set_filter(FilterMode::Nearest);

        let mut renderer = Renderer {
            render_target,
            width,
            height,
            scale: 1.0,
            camera,
        };
        renderer.set_scale();
        renderer
    }

    /// Set the camera to the render target. Should be called before drawing anything.
    pub fn set(&self) {
        set_camera(&self.camera);
    }

    /// Set the scale to fit the screen while maintaining the aspect ratio.
    fn set_scale(&mut self) {
        let hor_ratio = screen_width() / self.width;
        let ver_ratio = screen_height() / self.height;

        self.scale = if hor_ratio < ver_ratio {
            hor_ratio
        } else {
            ver_ratio
        };
    }

    /// Returns the offset to center the render target on the screen.
    fn screen_offset(&self) -> (f32, f32) {
        (
            (screen_width() - self.width * self.scale) / 2.0,
            (screen_height() - self.height * self.scale) / 2.0,
        )
    }

    /// Draw this render target to the screen with the correct aspect ratio and scaled to fit the screen.
    pub fn draw(&mut self) {
        set_default_camera();
        self.set_scale();
        let (offset_x, offset_y) = self.screen_offset();
        draw_texture_ex(
            self.render_target.texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.width * self.scale, self.height * self.scale)),
                ..Default::default()
            },
        )
    }

    /// Returns mouse position translated to the render target coordinates
    pub fn mouse_position(&self) -> (f32, f32) {
        let (mouse_x, mouse_y) = mouse_position();
        let (offset_x, offset_y) = self.screen_offset();
        (
            ((mouse_x - offset_x) / self.scale).floor(),
            ((mouse_y - offset_y) / self.scale).floor(),
        )
    }
}
