#version 300 es

precision highp float;

in vec3 colour;
in float time;
in vec2 resolution;
in vec2 mouse_position;

out vec4 frag_colour;

void main() {
	float time_variance = sin(time * 0.5);
	vec4 new_colour = vec4
	(
		colour[0] * time_variance, 
		colour[1] * time_variance, 
		colour[2] * time_variance, 
		1.0
	);
	frag_colour = new_colour;
}
