use glow::HasContext;
use glow::Context;

pub struct ShaderProgram {
    pub program: glow::Program,
    pub projection_location: glow::UniformLocation,
}

impl ShaderProgram {
    pub unsafe fn new(
        gl: &Context,
        vertex_src: &str,
        fragment_src: &str,
        projection_uniform: &str,
    ) -> Self {
        let vertex_shader = compile_shader(gl, glow::VERTEX_SHADER, vertex_src);
        let fragment_shader = compile_shader(gl, glow::FRAGMENT_SHADER, fragment_src);
        let program = gl.create_program().expect("create program");
        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, fragment_shader);
        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!(
                "Program link error: {}",
                gl.get_program_info_log(program)
            );
        }
        gl.delete_shader(vertex_shader);
        gl.delete_shader(fragment_shader);
        let projection_location = gl
            .get_uniform_location(program, projection_uniform)
            .expect("projection uniform");
        Self {
            program,
            projection_location,
        }
    }

    pub unsafe fn set_projection(&self, gl: &Context, matrix: &[f32; 16]) {
        gl.use_program(Some(self.program));
        gl.uniform_matrix_4_f32_slice(Some(&self.projection_location), false, matrix);
    }
}

pub struct TexturedShader {
    pub program: glow::Program,
    pub projection_location: glow::UniformLocation,
    pub texture_location: glow::UniformLocation,
}

impl TexturedShader {
    pub unsafe fn new(gl: &Context, vertex_src: &str, fragment_src: &str) -> Self {
        let vertex_shader = compile_shader(gl, glow::VERTEX_SHADER, vertex_src);
        let fragment_shader = compile_shader(gl, glow::FRAGMENT_SHADER, fragment_src);
        let program = gl.create_program().expect("create program");
        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, fragment_shader);
        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!(
                "Program link error: {}",
                gl.get_program_info_log(program)
            );
        }
        gl.delete_shader(vertex_shader);
        gl.delete_shader(fragment_shader);
        let projection_location = gl
            .get_uniform_location(program, "u_projection")
            .expect("u_projection");
        let texture_location = gl
            .get_uniform_location(program, "u_texture")
            .expect("u_texture");
        Self {
            program,
            projection_location,
            texture_location,
        }
    }

    pub unsafe fn set_projection(&self, gl: &Context, matrix: &[f32; 16]) {
        gl.use_program(Some(self.program));
        gl.uniform_matrix_4_f32_slice(Some(&self.projection_location), false, matrix);
    }

    pub unsafe fn bind_texture(&self, gl: &Context, unit: i32) {
        gl.use_program(Some(self.program));
        gl.uniform_1_i32(Some(&self.texture_location), unit);
    }
}

unsafe fn compile_shader(gl: &Context, shader_type: u32, source: &str) -> glow::Shader {
    let shader = gl.create_shader(shader_type).expect("create shader");
    gl.shader_source(shader, source);
    gl.compile_shader(shader);
    if !gl.get_shader_compile_status(shader) {
        panic!(
            "Shader compile error ({}): {}",
            shader_type,
            gl.get_shader_info_log(shader)
        );
    }
    shader
}
