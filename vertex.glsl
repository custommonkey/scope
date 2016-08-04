#version 140

in vec2 position;

uniform float t;
uniform float time;

void main() {
    vec2 pos = position;
    float x = time + pos.x;
    pos.y += (noise1(x * 23.3) / 5) + noise1(x) / 2;
    gl_Position = vec4(pos, 0.0, 1.0);
}
