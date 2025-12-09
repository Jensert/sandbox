#version 100
attribute vec3 position;
attribute vec2 texcoord;

varying lowp vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    uv = texcoord; // Pass the standard texture coordinates directly
    gl_Position = Projection * Model * vec4(position, 1);
}
