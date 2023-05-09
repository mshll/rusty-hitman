use macroquad::prelude::*;

pub const ARMS_COUNT: usize = 7;
pub const BODY_COUNT: usize = 10;
pub const FACE_COUNT: usize = 10;
pub const HAT_COUNT: usize = 7;
pub const LEGS_COUNT: usize = 6;
pub const CHAR_PARTS_COUNT: usize = 5;

/// Game assets.
pub struct AssetBundle {
    pub ground: Texture2D,
    pub bg: Texture2D,
    pub crosshair: Texture2D,
    pub char_arms: [Texture2D; ARMS_COUNT],
    pub char_body: [Texture2D; BODY_COUNT],
    pub char_face: [Texture2D; FACE_COUNT],
    pub char_hat: [Texture2D; HAT_COUNT],
    pub char_legs: [Texture2D; LEGS_COUNT],
    pub font: Font,
}

impl AssetBundle {
    /// Loads all game textures.
    pub async fn load() -> Result<AssetBundle, FileError> {
        let mut assets = AssetBundle {
            ground: load_texture("ground.png").await?,
            bg: load_texture("bg.png").await?,
            crosshair: load_texture("crosshair.png").await?,
            char_arms: [Texture2D::empty(); ARMS_COUNT],
            char_body: [Texture2D::empty(); BODY_COUNT],
            char_face: [Texture2D::empty(); FACE_COUNT],
            char_hat: [Texture2D::empty(); HAT_COUNT],
            char_legs: [Texture2D::empty(); LEGS_COUNT],
            font: load_ttf_font("ThaleahFat.ttf").await.unwrap(), // TODO: Handle error properly
        };

        // Set textures filters to nearest for better pixel art rendering.
        assets.ground.set_filter(FilterMode::Nearest);
        assets.bg.set_filter(FilterMode::Nearest);
        assets.crosshair.set_filter(FilterMode::Nearest);

        // Load character textures.
        for i in 0..ARMS_COUNT {
            assets.char_arms[i] =
                load_texture(format!("character/arms-{}.png", i).as_str()).await?;
            assets.char_arms[i].set_filter(FilterMode::Nearest);
        }

        for i in 0..BODY_COUNT {
            assets.char_body[i] =
                load_texture(format!("character/body-{}.png", i).as_str()).await?;
            assets.char_body[i].set_filter(FilterMode::Nearest);
        }

        for i in 0..FACE_COUNT {
            assets.char_face[i] =
                load_texture(format!("character/face-{}.png", i).as_str()).await?;
            assets.char_face[i].set_filter(FilterMode::Nearest);
        }

        for i in 0..HAT_COUNT {
            assets.char_hat[i] = load_texture(format!("character/hat-{}.png", i).as_str()).await?;
            assets.char_hat[i].set_filter(FilterMode::Nearest);
        }

        for i in 0..LEGS_COUNT {
            assets.char_legs[i] =
                load_texture(format!("character/legs-{}.png", i).as_str()).await?;
            assets.char_legs[i].set_filter(FilterMode::Nearest);
        }

        Ok(assets)
    }
}
