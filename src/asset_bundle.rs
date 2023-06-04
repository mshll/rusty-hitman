use macroquad::audio::*;
use macroquad::prelude::*;

pub const ARMS_COUNT: usize = 9;
pub const BODY_COUNT: usize = 10;
pub const FACE_COUNT: usize = 13;
pub const HAT_COUNT: usize = 10;
pub const LEGS_COUNT: usize = 5;
pub const CHAR_PARTS_COUNT: usize = 5;

/// Game assets.
pub struct AssetBundle {
    pub ground: Texture2D,
    pub crosshair: Texture2D,
    pub char_arms: [Texture2D; ARMS_COUNT],
    pub char_body: [Texture2D; BODY_COUNT],
    pub char_face: [Texture2D; FACE_COUNT],
    pub char_hat: [Texture2D; HAT_COUNT],
    pub char_legs: [Texture2D; LEGS_COUNT],
    pub logo: Texture2D,
    pub frame: Texture2D,
    pub frame_long: Texture2D,
    pub font: Font,
    pub bar: [Texture2D; 2],
    pub skull: Texture2D,
    pub blood: Texture2D,
    pub empty: Texture2D,
    pub bg_music: Sound,
    pub spawn_sound: Sound,
    pub shoot_sound: Sound,
    pub game_over_sound: Sound,
    pub menu_in_sound: Sound,
    pub menu_out_sound: Sound,
    pub pause_sound: Sound,
}

impl AssetBundle {
    /// Loads all game assets.
    pub async fn load() -> Result<AssetBundle, FileError> {
        let mut assets = AssetBundle {
            ground: load_texture("ground.png").await?,
            crosshair: load_texture("crosshair.png").await?,
            char_arms: [Texture2D::empty(); ARMS_COUNT],
            char_body: [Texture2D::empty(); BODY_COUNT],
            char_face: [Texture2D::empty(); FACE_COUNT],
            char_hat: [Texture2D::empty(); HAT_COUNT],
            char_legs: [Texture2D::empty(); LEGS_COUNT],
            logo: load_texture("logo.png").await?,
            frame: load_texture("frame.png").await?,
            frame_long: load_texture("frame-long.png").await?,
            font: load_ttf_font("04B03.TTF").await.unwrap(),
            bar: [
                load_texture("bar.png").await?,
                load_texture("bar-bg.png").await?,
            ],
            skull: load_texture("skull.png").await?,
            blood: load_texture("blood.png").await?,
            empty: load_texture("empty.png").await?,
            bg_music: load_sound("audio/puzzle_pieces.wav").await?,
            spawn_sound: load_sound("audio/spawn.wav").await?,
            shoot_sound: load_sound("audio/shoot.wav").await?,
            game_over_sound: load_sound("audio/evil_laugh.wav").await?,
            menu_in_sound: load_sound("audio/menu_in.wav").await?,
            menu_out_sound: load_sound("audio/menu_out.wav").await?,
            pause_sound: load_sound("audio/pause.wav").await?,
        };

        // Set textures filters to nearest for better pixel art rendering.
        assets.ground.set_filter(FilterMode::Nearest);
        assets.crosshair.set_filter(FilterMode::Nearest);
        assets.logo.set_filter(FilterMode::Nearest);
        assets.frame.set_filter(FilterMode::Nearest);
        assets.frame_long.set_filter(FilterMode::Nearest);
        assets.bar[0].set_filter(FilterMode::Nearest);
        assets.bar[1].set_filter(FilterMode::Nearest);
        assets.skull.set_filter(FilterMode::Nearest);
        assets.blood.set_filter(FilterMode::Nearest);
        assets.empty.set_filter(FilterMode::Nearest);

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
