/// Normalized UV rectangle (OpenGL texture space, origin bottom-left).
#[derive(Clone, Copy, Debug)]
pub struct UvRect {
    pub u0: f32,
    pub v0: f32,
    pub u1: f32,
    pub v1: f32,
}

pub struct SpriteAtlas {
    pub frames: Vec<UvRect>,
    pub tile_size: u32,
}

impl SpriteAtlas {
    pub fn from_grid(texture_width: u32, texture_height: u32, tile_width: u32, tile_height: u32) -> Self {
        if texture_width % tile_width != 0 || texture_height % tile_height != 0 {
            panic!("Tile size does not divide texture dimensions evenly");
        }

        let cols = texture_width / tile_width;
        let rows = texture_height / tile_height;
        let tw = texture_width as f32;
        let th = texture_height as f32;
        let tile_w = tile_width as f32;
        let tile_h = tile_height as f32;

        let mut frames = Vec::new();
        for row in 0..rows {
            for col in 0..cols {
                let px = col as f32 * tile_w;
                let py = row as f32 * tile_h;
                // PNG rows are top-down; convert to OpenGL bottom-left UVs.
                let u0 = px / tw;
                let u1 = (px + tile_w) / tw;
                let v1 = 1.0 - py / th;
                let v0 = 1.0 - (py + tile_h) / th;
                frames.push(UvRect { u0, v0, u1, v1 });
            }
        }

        Self {
            frames,
            tile_size: tile_width,
        }
    }
}
