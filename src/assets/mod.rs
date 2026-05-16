mod biome;
mod builder;
pub mod catalog;
mod embedded;
pub mod ids;

pub use biome::{Biome, BiomeCatalog};
pub use ids::{
    AssetId, RoadId, SkyId, CAR_FRAME_HEIGHT, CAR_FRAME_WIDTH,
};

use crate::graphics::{SpriteRegion, Texture2DArray};
use glow::Context;
use std::collections::HashMap;

pub struct Assets {
    pub skies: Texture2DArray,
    pub roads: Texture2DArray,
    pub sprites: Texture2DArray,
    pub car: Texture2DArray,
    pub sprite_regions: HashMap<AssetId, SpriteRegion>,
}

impl Assets {
    pub unsafe fn load(gl: &Context) -> Self {
        let built = builder::build(gl);
        Self {
            skies: built.skies,
            roads: built.roads,
            sprites: built.sprites,
            car: built.car,
            sprite_regions: built.sprite_regions,
        }
    }

    pub fn sky_layer(&self, id: SkyId) -> i32 {
        id.layer() as i32
    }

    pub fn road_layer(&self, id: RoadId) -> i32 {
        id.layer() as i32
    }
}
