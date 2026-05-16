use super::embedded::{self, SPRITE_ASSETS};
use super::ids::{AssetId, RoadId, SkyId, CAR_FRAME_COUNT};
use crate::graphics::texture_array::{pad_to_canvas, split_horizontal_strip};
use crate::graphics::{SpriteRegion, Texture2DArray};
use glow::Context;
use image::RgbaImage;
use std::collections::HashMap;

pub struct BuiltAssets {
    pub skies: Texture2DArray,
    pub roads: Texture2DArray,
    pub sprites: Texture2DArray,
    pub car: Texture2DArray,
    pub sprite_regions: HashMap<AssetId, SpriteRegion>,
}

fn decode(bytes: &[u8]) -> RgbaImage {
    image::load_from_memory(bytes)
        .expect("decode embedded png")
        .into_rgba8()
}

pub unsafe fn build(gl: &Context) -> BuiltAssets {
    let sky_layers: Vec<RgbaImage> = SkyId::ALL
        .iter()
        .map(|&id| decode(embedded::sky_bytes(id)))
        .collect();
    let skies = Texture2DArray::from_equal_layers(gl, &sky_layers);

    let road_layers: Vec<RgbaImage> = RoadId::ALL
        .iter()
        .map(|&id| decode(embedded::road_bytes(id)))
        .collect();
    let roads = Texture2DArray::from_equal_layers(gl, &road_layers);

    let mut max_w = 0u32;
    let mut max_h = 0u32;
    let mut decoded_sprites = Vec::new();
    for &id in SPRITE_ASSETS {
        let image = decode(embedded::asset_bytes(id));
        max_w = max_w.max(image.width());
        max_h = max_h.max(image.height());
        decoded_sprites.push((id, image));
    }

    let mut sprite_layers = Vec::new();
    let mut sprite_regions = HashMap::new();
    for (layer_index, (id, image)) in decoded_sprites.into_iter().enumerate() {
        let width = image.width();
        let height = image.height();
        sprite_layers.push(pad_to_canvas(&image, max_w, max_h));
        sprite_regions.insert(
            id,
            SpriteRegion {
                layer: layer_index as u32,
                width,
                height,
            },
        );
    }
    let sprites = Texture2DArray::from_equal_layers(gl, &sprite_layers);

    let car_strip = decode(embedded::car_bytes());
    let car_frames = split_horizontal_strip(&car_strip, CAR_FRAME_COUNT);
    let car = Texture2DArray::from_equal_layers(gl, &car_frames);

    BuiltAssets {
        skies,
        roads,
        sprites,
        car,
        sprite_regions,
    }
}
