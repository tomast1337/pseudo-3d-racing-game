use glow::HasContext;
use glow::Context;
use image::RgbaImage;
use std::path::Path;

pub struct GlTexture {
    pub id: glow::Texture,
    pub width: u32,
    pub height: u32,
}

impl GlTexture {
    pub unsafe fn from_image(gl: &Context, image: &RgbaImage) -> Self {
        let width = image.width();
        let height = image.height();
        let id = gl.create_texture().expect("create texture");
        gl.bind_texture(glow::TEXTURE_2D, Some(id));
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA as i32,
            width as i32,
            height as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            glow::PixelUnpackData::Slice(Some(&image)),
        );
        gl.bind_texture(glow::TEXTURE_2D, None);
        Self { id, width, height }
    }

    pub unsafe fn from_path(gl: &Context, path: impl AsRef<Path>) -> Result<Self, image::ImageError> {
        let image = image::open(path)?.into_rgba8();
        Ok(Self::from_image(gl, &image))
    }

    pub unsafe fn bind(&self, gl: &Context, unit: u32) {
        gl.active_texture(glow::TEXTURE0 + unit);
        gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
    }
}

impl Drop for GlTexture {
    fn drop(&mut self) {
        // GPU resources are freed when the glow Context is dropped.
    }
}
