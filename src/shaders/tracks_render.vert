#version 100

attribute vec2 in_pos;

uniform mat4 mvp;
uniform vec4 in_color;

varying lowp vec4 color;

void main() {
    gl_Position = mvp * vec4(in_pos, 0, 1);
    color = in_color;
}