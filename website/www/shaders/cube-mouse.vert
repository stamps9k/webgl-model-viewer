#version 300 es
in vec4 a_position;
in vec3 a_color;

uniform mat4 u_projection_matrix;
uniform mat4 u_camera_matrix;
uniform vec2 u_mouse_position;
uniform vec2 u_resolution;
uniform float u_time;

out vec3 colour;
out float time;
out vec2 mouse_position;
out vec2 resolution;

void main() {
	//Pass values to the frag shader
	colour = a_color;
	time = u_time;
	resolution = u_resolution;
	mouse_position = u_mouse_position;

	//Multiply the position by the projection matrix then the camera matrix	
	gl_Position = u_projection_matrix * u_camera_matrix * a_position;
}
