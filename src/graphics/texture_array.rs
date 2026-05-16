use glow::HasContext;
use glow::Context;
use image::RgbaImage;

pub struct Texture2DArray {
    pub id: glow::Texture,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

impl Texture2DArray {
    pub unsafe fn from_equal_layers(gl: &Context, layers: &[RgbaImage]) -> Self {
        assert!(!layers.is_empty(), "texture array needs at least one layer");
        let width = layers[0].width();
        let height = layers[0].height();
        for layer in layers {
            assert_eq!(
                (layer.width(), layer.height()),
                (width, height),
                "all layers must share dimensions"
            );
        }

        let mut pixels = Vec::with_capacity((width * height * 4 * layers.len() as u32) as usize);
        for layer in layers {
            pixels.extend_from_slice(layer.as_raw());
        }

        let id = gl.create_texture().expect("create texture array");
        gl.bind_texture(glow::TEXTURE_2D_ARRAY, Some(id));
        gl.tex_parameter_i32(
            glow::TEXTURE_2D_ARRAY,
            glow::TEXTURE_MIN_FILTER,
            glow::NEAREST as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D_ARRAY,
            glow::TEXTURE_MAG_FILTER,
            glow::NEAREST as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D_ARRAY,
            glow::TEXTURE_WRAP_S,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D_ARRAY,
            glow::TEXTURE_WRAP_T,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_image_3d(
            glow::TEXTURE_2D_ARRAY,
            0,
            glow::RGBA as i32,
            width as i32,
            height as i32,
            layers.len() as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            glow::PixelUnpackData::Slice(Some(&pixels)),
        );
        gl.bind_texture(glow::TEXTURE_2D_ARRAY, None);

        Self {
            id,
            width,
            height,
            layers: layers.len() as u32,
        }
    }

    pub unsafe fn bind(&self, gl: &Context, unit: u32) {
        gl.active_texture(glow::TEXTURE0 + unit);
        gl.bind_texture(glow::TEXTURE_2D_ARRAY, Some(self.id));
    }
}

/// Place `image` at the bottom-left of a transparent `canvas_w` × `canvas_h` layer.
pub fn pad_to_canvas(image: &RgbaImage, canvas_w: u32, canvas_h: u32) -> RgbaImage {
    let mut canvas = RgbaImage::new(canvas_w, canvas_h);
    let x0 = 0i64;
    let y0 = canvas_h.saturating_sub(image.height()) as i64;
    image::imageops::overlay(&mut canvas, image, x0, y0);
    canvas
}

/// Split a horizontal strip into `frame_count` equal-width layers.
pub fn split_horizontal_strip(image: &RgbaImage, frame_count: u32) -> Vec<RgbaImage> {
    assert!(
        image.width() % frame_count == 0,
        "strip width {} not divisible by frame count {}",
        image.width(),
        frame_count
    );
    let frame_w = image.width() / frame_count;
    let frame_h = image.height();
    (0..frame_count)
        .map(|i| image::imageops::crop_imm(image, i * frame_w, 0, frame_w, frame_h).to_image())
        .collect()
}
