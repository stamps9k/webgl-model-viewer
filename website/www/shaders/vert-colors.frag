#version 300 es

precision highp float;

in vec3 colour;
out vec4 frag_colour;

void main() {
	frag_colour = vec4(colour, 1.0);
}
