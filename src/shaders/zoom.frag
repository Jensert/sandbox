
#version 100
precision lowp float;

varying vec2 uv; // normalized texture coordinates. passed in from the vertex shader
// Uniforms
uniform sampler2D _ScreenTexture;
uniform vec2 MousePosition; // mouse position in pixels
uniform float Zoom;        // zoom factor > 1.0 = zoom in

// Screen resolution
vec2 screenSize = vec2(320.0, 180.0);

void main() {
    // Compute mouse position in UV space (0–1)
    vec2 mouseUV = MousePosition / screenSize;
    mouseUV.y = 1.0 - mouseUV.y; // Flip Y to match texture space
    // Zoom around mouse:
    //   Move uv relative to mouse → scale → move back
    vec2 zoomedUV = (uv - mouseUV) / Zoom + mouseUV;

    gl_FragColor = texture2D(_ScreenTexture, zoomedUV);
}
