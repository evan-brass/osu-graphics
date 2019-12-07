#version 330 compatibility

in gData {
    vec3 color;
} vert;

out vec4 FragColor;

void main() {
    FragColor = vec4(vert.color, 1.0);
} 