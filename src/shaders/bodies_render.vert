#version 100

attribute vec2 in_vert_pos;
attribute vec2 in_body_pos;
attribute vec4 in_color;

varying lowp vec4 color;

uniform mat4 mvp;

void main() {
    vec4 pos = vec4(in_vert_pos + in_body_pos, 0, 1);
    gl_Position = mvp * pos;
    color = in_color;
}