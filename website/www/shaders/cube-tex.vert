#version 300 es
in vec4 a_position;
in vec2 a_texcoord;

uniform mat4 u_projection_matrix;
uniform mat4 u_camera_matrix;
uniform vec2 u_mouse_position;
uniform vec2 u_resolution;
uniform float u_time;

out vec2 v_texcoord;

void main() {
	//Multiply the position by the projection matrix then the camera matrix	
	gl_Position = u_projection_matrix * u_camera_matrix * a_position;

	//Pass the texture co-ordinate to the fragment shader	
	v_texcoord = a_texcoord;
}
