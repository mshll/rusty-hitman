use macroquad::prelude::*;

pub const ARMS_COUNT: usize = 7;
pub const BODY_COUNT: usize = 10;
pub const FACE_COUNT: usize = 10;
pub const HAT_COUNT: usize = 7;
pub const LEGS_COUNT: usize = 6;
pub const CHAR_PARTS_COUNT: usize = 5;

/// Game textures atlas.
pub struct TextureAtlas {
    pub ground: Texture2D,
    pub bg: Texture2D,
    pub crosshair: Texture2D,
    pub char_arms: [Texture2D; ARMS_COUNT],
    pub char_body: [Texture2D; BODY_COUNT],
    pub char_face: [Texture2D; FACE_COUNT],
    pub char_hat: [Texture2D; HAT_COUNT],
    pub char_legs: [Texture2D; LEGS_COUNT],
}

impl TextureAtlas {
    /// Loads all game textures and builds the atlas.
    pub async fn load() -> Result<TextureAtlas, FileError> {
        let mut atlas = TextureAtlas {
            ground: load_texture("ground.png").await?,
            bg: load_texture("bg.png").await?,
            crosshair: load_texture("crosshair.png").await?,
            char_arms: [Texture2D::empty(); ARMS_COUNT],
            char_body: [Texture2D::empty(); BODY_COUNT],
            char_face: [Texture2D::empty(); FACE_COUNT],
            char_hat: [Texture2D::empty(); HAT_COUNT],
            char_legs: [Texture2D::empty(); LEGS_COUNT],
        };

        // Set textures filters to nearest for better pixel art rendering.
        atlas.ground.set_filter(FilterMode::Nearest);
        atlas.bg.set_filter(FilterMode::Nearest);
        atlas.crosshair.set_filter(FilterMode::Nearest);

        // Load character textures.
        for i in 0..ARMS_COUNT {
            atlas.char_arms[i] = load_texture(format!("character/arms-{}.png", i).as_str()).await?;
            atlas.char_arms[i].set_filter(FilterMode::Nearest);
        }

        for i in 0..BODY_COUNT {
            atlas.char_body[i] = load_texture(format!("character/body-{}.png", i).as_str()).await?;
            atlas.char_body[i].set_filter(FilterMode::Nearest);
        }

        for i in 0..FACE_COUNT {
            atlas.char_face[i] = load_texture(format!("character/face-{}.png", i).as_str()).await?;
            atlas.char_face[i].set_filter(FilterMode::Nearest);
        }

        for i in 0..HAT_COUNT {
            atlas.char_hat[i] = load_texture(format!("character/hat-{}.png", i).as_str()).await?;
            atlas.char_hat[i].set_filter(FilterMode::Nearest);
        }

        for i in 0..LEGS_COUNT {
            atlas.char_legs[i] = load_texture(format!("character/legs-{}.png", i).as_str()).await?;
            atlas.char_legs[i].set_filter(FilterMode::Nearest);
        }

        Ok(atlas)
    }
}
