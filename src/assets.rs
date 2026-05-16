use crate::graphics::{GlTexture, SpriteAtlas};
use glow::Context;
use std::path::PathBuf;

pub const TILE_SIZE: u32 = 64;
pub const PLAYER_TEXTURE: &str = "assets/textures/Player.png";

pub struct Assets {
    pub player_texture: GlTexture,
    pub player_atlas: SpriteAtlas,
}

impl Assets {
    pub unsafe fn load(gl: &Context) -> Result<Self, image::ImageError> {
        let path = asset_path(PLAYER_TEXTURE);
        let player_texture = GlTexture::from_path(gl, &path)?;
        let player_atlas = SpriteAtlas::from_grid(
            player_texture.width,
            player_texture.height,
            TILE_SIZE,
            TILE_SIZE,
        );
        Ok(Self {
            player_texture,
            player_atlas,
        })
    }
}

pub fn asset_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative)
}
