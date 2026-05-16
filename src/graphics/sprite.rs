/// Normalized UV rectangle (OpenGL texture space, origin bottom-left).
#[derive(Clone, Copy, Debug)]
pub struct UvRect {
    pub u0: f32,
    pub v0: f32,
    pub u1: f32,
    pub v1: f32,
}

impl UvRect {
    /// Full layer UVs matching PNG orientation (row 0 = top of image).
    pub const FULL: Self = Self {
        u0: 0.0,
        v0: 0.0,
        u1: 1.0,
        v1: 1.0,
    };

    /// One horizontal row in a 128px-tall road depth texture.
    pub fn road_row(row: f32, scroll: f32, rows: f32) -> Self {
        let v = ((scroll * rows) + row).fract();
        let half = 0.5 / rows;
        Self {
            u0: 0.0,
            v0: v,
            u1: 1.0,
            v1: (v + 2.0 * half).min(1.0),
        }
    }
}

/// Metadata for a sprite stored in a texture array layer (may be padded).
#[derive(Clone, Copy, Debug)]
pub struct SpriteRegion {
    pub layer: u32,
    pub width: u32,
    pub height: u32,
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

    /// Horizontal strip: frame 0 is leftmost in the image.
    pub fn from_horizontal_strip(texture_width: u32, texture_height: u32, frame_count: u32) -> Self {
        assert!(
            texture_width % frame_count == 0,
            "frame count {frame_count} does not divide width {texture_width}"
        );
        let frame_width = texture_width / frame_count;
        Self::from_grid(texture_width, texture_height, frame_width, texture_height)
    }
}
