#version 100
precision lowp float;

varying vec2 uv; // normalized texture coordinates. passed in from the vertex shader
uniform sampler2D _ScreenTexture; // _ScreenTexture is macroquad internal uniform.
                                  // The last rendered frame is automatically supplied as the texture here

void main() {
    vec4 pixelColor = texture2D(_ScreenTexture, uv);
    gl_FragColor = pixelColor;
}
