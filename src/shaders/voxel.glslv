#version 330 core

in float size;
in vec3 color;

// Encodes the translation of this chunk in the scene as well as the scale of the chunk
uniform mat4 chunk_transform;

out vData {
    float size;
    vec3 color;
	vec4 position;
} vert_out;

const int WIDTH = 5;
const int HEIGHT = 5;
const int DEPTH = 5;
const int DEP_HEI = DEPTH * HEIGHT;

void main() {
	// Calculate the x, y, and z from the index based on the number of voxels that are in our chunk.
	int z = int(mod(gl_VertexID, DEPTH));
	int y = int(mod(floor(gl_VertexID / DEPTH), HEIGHT));
	int x = int(floor(gl_VertexID / DEP_HEI));

    vert_out.position = vec4(x, y, z, 1.0) * chunk_transform;
    vert_out.color = vec3(gl_VertexID, gl_VertexID, gl_VertexID);
	vert_out.size = size;
}