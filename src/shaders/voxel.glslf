#version 330 core

in gData {
    vec3 color;
} vert;

layout(location = 0) out vec4 diffuseColor;

void main() {
    diffuseColor = vec4(vert.color, 1.0);
}