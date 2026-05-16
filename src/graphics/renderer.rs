use super::shader::{ShaderProgram, TexturedShader};
use super::sprite::UvRect;
use super::texture::GlTexture;
use super::vertex::{ColoredVertex, TexturedVertex};
use bytemuck;
use glow::Context;
use glow::HasContext;

/// Screen-space orthographic projection: origin top-left, +y downward.
pub fn ortho_projection(width: f32, height: f32) -> [f32; 16] {
    [
        2.0 / width,
        0.0,
        0.0,
        0.0,
        0.0,
        -2.0 / height,
        0.0,
        0.0,
        0.0,
        0.0,
        -1.0,
        0.0,
        -1.0,
        1.0,
        0.0,
        1.0,
    ]
}

pub struct Renderer {
    pub width: f32,
    pub height: f32,
    pub projection: [f32; 16],
    colored_shader: ShaderProgram,
    textured_shader: TexturedShader,
    colored_vao: glow::VertexArray,
    colored_vbo: glow::Buffer,
    textured_vao: glow::VertexArray,
    textured_vbo: glow::Buffer,
}

impl Renderer {
    pub unsafe fn new(gl: &Context, width: f32, height: f32) -> Self {
        let colored_shader = ShaderProgram::new(
            gl,
            include_str!("shaders/colored.vert"),
            include_str!("shaders/colored.frag"),
            "u_projection",
        );
        let textured_shader = TexturedShader::new(
            gl,
            include_str!("shaders/textured.vert"),
            include_str!("shaders/textured.frag"),
        );

        let colored_vao = gl.create_vertex_array().expect("colored vao");
        let colored_vbo = gl.create_buffer().expect("colored vbo");
        gl.bind_vertex_array(Some(colored_vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(colored_vbo));
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 24, 0);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, 24, 8);
        gl.bind_vertex_array(None);

        let textured_vao = gl.create_vertex_array().expect("textured vao");
        let textured_vbo = gl.create_buffer().expect("textured vbo");
        gl.bind_vertex_array(Some(textured_vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(textured_vbo));
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 16, 0);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 16, 8);
        gl.bind_vertex_array(None);

        gl.enable(glow::BLEND);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

        let projection = ortho_projection(width, height);
        Self {
            width,
            height,
            projection,
            colored_shader,
            textured_shader,
            colored_vao,
            colored_vbo,
            textured_vao,
            textured_vbo,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.projection = ortho_projection(width, height);
    }

    pub unsafe fn clear(&self, gl: &Context, r: f32, g: f32, b: f32) {
        gl.viewport(0, 0, self.width as i32, self.height as i32);
        gl.clear_color(r, g, b, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);
    }

    pub unsafe fn draw_colored_rect(
        &self,
        gl: &Context,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: [f32; 4],
    ) {
        let vertices = [
            ColoredVertex {
                position: [x, y],
                color,
            },
            ColoredVertex {
                position: [x + w, y],
                color,
            },
            ColoredVertex {
                position: [x + w, y + h],
                color,
            },
            ColoredVertex {
                position: [x, y],
                color,
            },
            ColoredVertex {
                position: [x + w, y + h],
                color,
            },
            ColoredVertex {
                position: [x, y + h],
                color,
            },
        ];
        gl.use_program(Some(self.colored_shader.program));
        self.colored_shader.set_projection(gl, &self.projection);
        gl.bind_vertex_array(Some(self.colored_vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.colored_vbo));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            bytemuck::cast_slice(&vertices),
            glow::DYNAMIC_DRAW,
        );
        gl.draw_arrays(glow::TRIANGLES, 0, 6);
        gl.bind_vertex_array(None);
    }

    pub unsafe fn draw_textured_quad(
        &self,
        gl: &Context,
        texture: &GlTexture,
        center_x: f32,
        center_y: f32,
        width: f32,
        height: f32,
        uv: UvRect,
    ) {
        let half_w = width * 0.5;
        let half_h = height * 0.5;
        let left = center_x - half_w;
        let right = center_x + half_w;
        let top = center_y - half_h;
        let bottom = center_y + half_h;

        let vertices = [
            TexturedVertex {
                position: [left, top],
                uv: [uv.u0, uv.v1],
            },
            TexturedVertex {
                position: [right, top],
                uv: [uv.u1, uv.v1],
            },
            TexturedVertex {
                position: [right, bottom],
                uv: [uv.u1, uv.v0],
            },
            TexturedVertex {
                position: [left, top],
                uv: [uv.u0, uv.v1],
            },
            TexturedVertex {
                position: [right, bottom],
                uv: [uv.u1, uv.v0],
            },
            TexturedVertex {
                position: [left, bottom],
                uv: [uv.u0, uv.v0],
            },
        ];

        gl.use_program(Some(self.textured_shader.program));
        self.textured_shader.set_projection(gl, &self.projection);
        texture.bind(gl, 0);
        self.textured_shader.bind_texture(gl, 0);
        gl.bind_vertex_array(Some(self.textured_vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.textured_vbo));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            bytemuck::cast_slice(&vertices),
            glow::DYNAMIC_DRAW,
        );
        gl.draw_arrays(glow::TRIANGLES, 0, 6);
        gl.bind_vertex_array(None);
        gl.bind_texture(glow::TEXTURE_2D, None);
    }
}
