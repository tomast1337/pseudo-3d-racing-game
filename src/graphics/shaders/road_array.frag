#version 330 core

in float v_screen_s;
in float v_uv_v;
in float v_clip_lo;
in float v_clip_hi;

out vec4 frag_color;

uniform sampler2DArray u_texture;
uniform int u_layer;
uniform float u_tex_inner_lo;
uniform float u_tex_inner_hi;

/// Left/right of screen: repeat edge texels (u=0 / u=1). Between clip bounds: map road band.
float remap_road_u(float s, float clip_lo, float clip_hi) {
    if (s <= clip_lo) {
        return 0.0;
    }
    if (s >= clip_hi) {
        return 1.0;
    }
    float t = (s - clip_lo) / (clip_hi - clip_lo);
    return mix(u_tex_inner_lo, u_tex_inner_hi, t);
}

void main() {
    float u = remap_road_u(v_screen_s, v_clip_lo, v_clip_hi);
    frag_color = texture(u_texture, vec3(u, v_uv_v, float(u_layer)));
    if (frag_color.a < 0.01) {
        discard;
    }
}
