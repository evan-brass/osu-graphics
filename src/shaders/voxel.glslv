#version 330 compatibility

in float size;
in vec3 color;

// Encodes the translation of this chunk in the scene as well as the scale of the chunk
uniform mat4 chunk_transform;

out vData {
    float size;
    vec3 color;
	vec4 position;
} vert_out;

const int WIDTH = 20;
const int HEIGHT = 60;
const int DEPTH = 20;
const int DEP_HEI = DEPTH * HEIGHT;

void main() {
	// Calculate the x, y, and z from the index based on the number of voxels that are in our chunk.
	int z = int(mod(gl_VertexID, DEPTH));
	int y = int(mod(floor(gl_VertexID / DEPTH), HEIGHT));
	int x = int(floor(gl_VertexID / DEP_HEI));

    vert_out.position = chunk_transform * vec4(x, y, z, 1.0);
    vert_out.color = color;
	vert_out.size = size;
}