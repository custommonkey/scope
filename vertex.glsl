#version 140

in vec2 position;

uniform float t;
uniform float time;

void main() {
    vec2 pos = position;
    pos.y += noise1(time + pos.x) / 2;
    gl_Position = vec4(pos, 0.0, 1.0);
}
