#version 300 es
in vec4 a_position;
in vec3 a_color;
uniform mat4 u_matrix;

out vec3 colour;

void main() {
		colour = a_color;
		gl_Position = u_matrix * a_position;
		//gl_Position = a_position;
}
