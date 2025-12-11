#version 100
precision mediump float;

varying vec2 uv; // normalized texture coordinates. passed in from the vertex shader
uniform float _Time;
uniform sampler2D _ScreenTexture;

vec4 LAVA = vec4(0.70, 0.16, 0.22, 1.00);
vec4 STONE = vec4(0.51, 0.51, 0.51, 1.00);
vec4 DEEPSTONE = vec4(0.31, 0.31, 0.31, 1.00);
vec4 GRASS = vec4(0.00, 0.46, 0.17, 1.00);
vec4 WATER = vec4(0.00, 0.47, 0.95, 1.00);
vec4 AIR = vec4(0.40, 0.75, 1.00, 1.00);
vec4 SAND = vec4(0.83, 0.69, 0.51, 1.00);
vec4 DIRT = vec4(0.30, 0.25, 0.18, 1.00);

uniform sampler2D Tex_Stone;
uniform sampler2D Tex_Lava;
uniform sampler2D Tex_DeepStone;
uniform sampler2D Tex_Grass;
uniform sampler2D Tex_Water;
uniform sampler2D Tex_Air;
uniform sampler2D Tex_Sand;
uniform sampler2D Tex_Dirt;


// Simple hash function for per-pixel randomness
float hash(vec2 p) {
    return fract(sin(dot(p ,vec2(127.1,311.7))) * 43758.5453123);
}

// Screen resolution
vec2 screenSize = vec2(360.0, 180.0);

// --------------------- Materials ---------------------

vec4 drawStone(vec2 uv) {
    vec2 pixelUV = floor(uv * screenSize);
    float rnd = hash(pixelUV);
    vec3 base = vec3(0.51, 0.51, 0.51);
    float variation = (rnd - 0.5) * 0.1;
    return vec4(base + variation, 1.0);
}

vec4 drawDeepStone(vec2 uv) {
    vec2 pixelUV = floor(uv * screenSize);
    float rnd = hash(pixelUV + 10.0); // offset for different pattern
    vec3 base = vec3(0.31, 0.31, 0.31);
    float variation = (rnd - 0.5) * 0.12;
    return vec4(base + variation, 1.0);
}

vec4 drawLava(vec2 uv) {
    vec2 pixelUV = floor(uv * screenSize);
    float rnd = hash(pixelUV + 20.0);
    vec3 base = vec3(0.70, 0.16, 0.22);
    float variation = (rnd - 0.5) * 0.12;
    return vec4(base + variation, 1.0);
}

vec4 drawGrass(vec2 uv) {
    vec2 pixelUV = floor(uv * screenSize);
    float rnd = hash(pixelUV + 30.0);
    vec3 base = vec3(0.00, 0.46, 0.17);
    float variation = (rnd - 0.5) * 0.08;
    return vec4(base + variation, 1.0);
}

vec4 drawWater(vec2 uv) {
    vec2 pixelUV = floor(uv * screenSize);
    float rnd = hash(pixelUV + 40.0);
    vec3 base = vec3(0.00, 0.47, 0.95);
    float variation = (rnd - 0.5) * 0.08;
    return vec4(base + variation, 1.0);
}

vec4 drawAir(vec2 uv) {
    vec2 pixelUV = floor(uv * screenSize);
    float rnd = hash(pixelUV + 50.0);
    vec3 base = vec3(0.40, 0.75, 1.00);
    float variation = (rnd - 0.5) * 0.03;
    return vec4(base + variation, 1.0);
}

vec4 drawSand(vec2 uv) {
    vec2 pixelUV = floor(uv * screenSize);
    float rnd = hash(pixelUV + 60.0);
    vec3 base = vec3(0.83, 0.69, 0.51);
    float variation = (rnd - 0.5) * 0.06;
    return vec4(base + variation, 1.0);
}

vec4 drawDirt(vec2 uv) {
    vec2 pixelUV = floor(uv * screenSize);
    float rnd = hash(pixelUV + 70.0);
    vec3 base = vec3(0.30, 0.25, 0.18);
    float variation = (rnd - 0.5) * 0.06;
    return vec4(base + variation, 1.0);
}

vec4 sampleColor(vec2 uv, vec4 color) {
    float tolerance = 0.05;

    if (distance(color, STONE) < tolerance)      return drawStone(uv);
    if (distance(color, LAVA) < tolerance)       return drawLava(uv);
    if (distance(color, DEEPSTONE) < tolerance)  return drawDeepStone(uv);
    if (distance(color, GRASS) < tolerance)      return drawGrass(uv);
    if (distance(color, WATER) < tolerance)      return drawWater(uv);
    if (distance(color, AIR) < tolerance)        return drawAir(uv);
    if (distance(color, SAND) < tolerance)       return drawSand(uv);
    if (distance(color, DIRT) < tolerance)       return drawDirt(uv);

    return vec4(0.0);
}

void main() {
    vec4 screenColor = texture2D(_ScreenTexture, uv);
    gl_FragColor = sampleColor(uv, screenColor);
}
