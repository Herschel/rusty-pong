#version 140

in vec4 v_color;    // Input color from vertex shader
out vec4 f_color;   // Output fragment color 

void main() {
    // Output polygon color.
    f_color = v_color;
}
