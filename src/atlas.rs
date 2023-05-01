use macroquad::prelude::*;

/// Game textures atlas.
pub struct TextureAtlas {
    pub ground: Texture2D,
    pub bg: Texture2D,
    pub char_body: [Vec<Texture2D>; 4], // top, bottom, left, right
    pub crosshair: Texture2D,
}

impl TextureAtlas {
    /// Loads all game textures and builds the atlas.
    /// TODO: maybe use array instead of a vector for char_body
    pub async fn init() -> TextureAtlas {
        let mut atlas = TextureAtlas {
            ground: load_texture("ground.png").await.unwrap(),
            bg: load_texture("bg.png").await.unwrap(),
            char_body: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            crosshair: load_texture("crosshair.png").await.unwrap(),
        };

        // Load character body textures.
        for i in 0..=4 {
            atlas.char_body[0].push(
                load_texture(format!("character/top{}.png", i).as_str())
                    .await
                    .unwrap(),
            );
            atlas.char_body[1].push(
                load_texture(format!("character/btm{}.png", i).as_str())
                    .await
                    .unwrap(),
            );
            atlas.char_body[2].push(
                load_texture(format!("character/left{}.png", i).as_str())
                    .await
                    .unwrap(),
            );
            atlas.char_body[3].push(
                load_texture(format!("character/right{}.png", i).as_str())
                    .await
                    .unwrap(),
            );
        }

        build_textures_atlas();
        atlas
    }
}
