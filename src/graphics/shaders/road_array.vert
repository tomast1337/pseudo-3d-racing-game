#version 330 core

layout(location = 0) in vec2 a_position;
layout(location = 1) in float a_v;
layout(location = 2) in float a_clip_lo;
layout(location = 3) in float a_clip_hi;

uniform mat4 u_projection;
uniform float u_screen_width;
uniform float u_shift;

out float v_screen_s;
out float v_uv_v;
out float v_clip_lo;
out float v_clip_hi;

void main() {
    v_screen_s = a_position.x / u_screen_width + u_shift;
    v_uv_v = a_v;
    v_clip_lo = a_clip_lo;
    v_clip_hi = a_clip_hi;
    gl_Position = u_projection * vec4(a_position, 0.0, 1.0);
}
