#version 330 core

in vec2 v_uv;
out vec4 frag_color;

uniform sampler2DArray u_texture;
uniform int u_layer;

void main() {
    frag_color = texture(u_texture, vec3(v_uv, float(u_layer)));
    if (frag_color.a < 0.01) {
        discard;
    }
}
