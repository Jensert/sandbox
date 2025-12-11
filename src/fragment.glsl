#version 100
precision lowp float;

varying vec2 uv; // normalized texture coordinates. passed in from the vertex shader
uniform sampler2D Texture; // Texture is macroquad internal uniform.
                           // Everything rendered after gl_set_material() will be part of the Texture uniform
                           // _ScreenTexture is everything that is already rendered on the screen
                           // So before gl_set_material()
                           // 

vec4 LAVA = vec4(0.90, 0.16, 0.22, 1.00);
vec4 STONE = vec4(0.51, 0.51, 0.51, 1.00);
vec4 DEEPSTONE = vec4(0.31, 0.31, 0.31, 1.00);
vec4 GRASS = vec4(0.00, 0.46, 0.17, 1.00);
vec4 WATER = vec4(0.00, 0.47, 0.95, 1.00);
vec4 AIR = vec4(0.40, 0.75, 1.00, 1.00);
vec4 SAND = vec4(0.83, 0.69, 0.51, 1.00);
vec4 DIRT = vec4(0.30, 0.25, 0.18, 1.00);

void main() {
    vec4 colorIn = texture2D(Texture, uv);
    float tolerance = 0.05;
    bool isMatch =
        distance(colorIn, LAVA) < tolerance ||
        distance(colorIn, STONE) < tolerance ||
        distance(colorIn, DEEPSTONE) < tolerance ||
        distance(colorIn, GRASS) < tolerance ||
        distance(colorIn, WATER) < tolerance ||
        distance(colorIn, AIR) < tolerance ||
        distance(colorIn, SAND) < tolerance ||
        distance(colorIn, DIRT) < tolerance;

        if (isMatch) {
        gl_FragColor = colorIn;
    }
}
