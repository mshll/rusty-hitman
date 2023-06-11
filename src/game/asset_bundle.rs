use super::utils::colors::*;
use super::utils::text::*;
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

enum Asset {
    Texture(Texture2D),
    Sound(Sound),
}

impl AssetBundle {
    /// Loads all game assets.
    pub async fn load() -> Option<AssetBundle> {
        let font = load_ttf_font("04B03.TTF").await.unwrap();
        let mut asset_paths = vec![
            "images/ground.png".to_string(),
            "images/crosshair.png".to_string(),
            "images/logo.png".to_string(),
            "images/frame.png".to_string(),
            "images/frame-long.png".to_string(),
            "images/bar.png".to_string(),
            "images/bar-bg.png".to_string(),
            "images/skull.png".to_string(),
            "images/blood.png".to_string(),
            "images/empty.png".to_string(),
            "audio/puzzle_pieces.wav".to_string(),
            "audio/spawn.wav".to_string(),
            "audio/shoot.wav".to_string(),
            "audio/evil_laugh.wav".to_string(),
            "audio/menu_in.wav".to_string(),
            "audio/menu_out.wav".to_string(),
            "audio/pause.wav".to_string(),
        ];
        for i in 0..ARMS_COUNT {
            asset_paths.push(format!("images/character/arms-{}.png", i));
        }
        for i in 0..BODY_COUNT {
            asset_paths.push(format!("images/character/body-{}.png", i));
        }
        for i in 0..FACE_COUNT {
            asset_paths.push(format!("images/character/face-{}.png", i));
        }
        for i in 0..HAT_COUNT {
            asset_paths.push(format!("images/character/hat-{}.png", i));
        }
        for i in 0..LEGS_COUNT {
            asset_paths.push(format!("images/character/legs-{}.png", i));
        }

        // Load all assets in parallel using coroutines.
        // - Thanks to Osennyaya#4019 for code example.
        let mut assets_vec: Vec<Asset> = Vec::with_capacity(asset_paths.len());
        let coroutine_vec: Vec<_> = asset_paths
            .into_iter()
            .map(|p| coroutines::start_coroutine(load_asset(p)))
            .collect();

        for c in coroutine_vec {
            while !c.is_done() {
                clear_background(BG_PURPLE);
                draw_text_centered(
                    "Loading...",
                    screen_width() / 2.0,
                    screen_height() / 2.0,
                    font,
                    64,
                    WHITE,
                );
                next_frame().await;
            }
            if let Some(asset) = c.retrieve() {
                assets_vec.push(asset);
            }
        }

        assets_vec.reverse();
        let mut assets = AssetBundle {
            ground: pop_texture(&mut assets_vec)?,
            crosshair: pop_texture(&mut assets_vec)?,
            logo: pop_texture(&mut assets_vec)?,
            frame: pop_texture(&mut assets_vec)?,
            frame_long: pop_texture(&mut assets_vec)?,
            bar: [pop_texture(&mut assets_vec)?, pop_texture(&mut assets_vec)?],
            skull: pop_texture(&mut assets_vec)?,
            blood: pop_texture(&mut assets_vec)?,
            empty: pop_texture(&mut assets_vec)?,
            bg_music: pop_sound(&mut assets_vec)?,
            spawn_sound: pop_sound(&mut assets_vec)?,
            shoot_sound: pop_sound(&mut assets_vec)?,
            game_over_sound: pop_sound(&mut assets_vec)?,
            menu_in_sound: pop_sound(&mut assets_vec)?,
            menu_out_sound: pop_sound(&mut assets_vec)?,
            pause_sound: pop_sound(&mut assets_vec)?,
            font,
            char_arms: [Texture2D::empty(); ARMS_COUNT],
            char_body: [Texture2D::empty(); BODY_COUNT],
            char_face: [Texture2D::empty(); FACE_COUNT],
            char_hat: [Texture2D::empty(); HAT_COUNT],
            char_legs: [Texture2D::empty(); LEGS_COUNT],
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

        for i in 0..ARMS_COUNT {
            assets.char_arms[i] = pop_texture(&mut assets_vec)?;
            assets.char_arms[i].set_filter(FilterMode::Nearest);
        }
        for i in 0..BODY_COUNT {
            assets.char_body[i] = pop_texture(&mut assets_vec)?;
            assets.char_body[i].set_filter(FilterMode::Nearest);
        }
        for i in 0..FACE_COUNT {
            assets.char_face[i] = pop_texture(&mut assets_vec)?;
            assets.char_face[i].set_filter(FilterMode::Nearest);
        }
        for i in 0..HAT_COUNT {
            assets.char_hat[i] = pop_texture(&mut assets_vec)?;
            assets.char_hat[i].set_filter(FilterMode::Nearest);
        }
        for i in 0..LEGS_COUNT {
            assets.char_legs[i] = pop_texture(&mut assets_vec)?;
            assets.char_legs[i].set_filter(FilterMode::Nearest);
        }

        Some(assets)
    }
}

/// Pop a texture from the assets vector.
fn pop_texture(assets: &mut Vec<Asset>) -> Option<Texture2D> {
    match assets.pop().unwrap() {
        Asset::Texture(t) => Some(t),
        _ => None,
    }
}

/// Pop a sound from the assets vector.
fn pop_sound(assets: &mut Vec<Asset>) -> Option<Sound> {
    match assets.pop().unwrap() {
        Asset::Sound(s) => Some(s),
        _ => None,
    }
}

/// Load asset from the given path.
async fn load_asset(path: String) -> Asset {
    let ext = path.split('.').last().unwrap();
    match ext {
        "png" => Asset::Texture(load_texture(path.as_str()).await.unwrap()),
        "wav" => Asset::Sound(load_sound(path.as_str()).await.unwrap()),
        _ => panic!("Unknown file extension"),
    }
}
