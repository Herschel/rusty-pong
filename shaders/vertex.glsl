#version 140

in vec2 position;
uniform vec4 color;
uniform mat4x4 transform;
uniform mat4x4 projection;

out vec4 v_color;

void main() {
    // Convert the 2D coordinates to 4D homogenous coordinates and transform.
    gl_Position = projection * transform * vec4(position, 0.0, 1.0);
    
    // Copy input polygon color to vertex.
    v_color = color;
}
