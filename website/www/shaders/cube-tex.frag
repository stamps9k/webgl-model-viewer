#version 300 es
precision highp float;

//Passed in from the vertex shader
in vec2 v_texcoord;

//The texture
uniform sampler2D u_texture;

out vec4 frag_colour;

void main() {
	frag_colour = texture(u_texture, v_texcoord);
}
