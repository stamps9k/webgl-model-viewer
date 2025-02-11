mod controller;
mod shaders;
mod utils;
mod object_loader;
mod logger;
mod webgl;

use crate::controller::*;
use crate::shaders::*;
use crate::logger::*;
use crate::webgl::*;
use crate::utils::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use webgl_matrix::*;
use web_sys::*;
use js_sys::Map;
use std::f64::consts::*;
use std::sync::Arc;
use std::sync::Mutex;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn initialize_web_gl(resources: Map) -> Result<(), JsValue> 
{
	logger::rust_info(&"Initialising webgl...");
	let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("glCanvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap();
    let mut frame = WebGl2Frame 
	{
		context: canvas.get_context("webgl2")?.unwrap().dyn_into::<web_sys::WebGl2RenderingContext>()?,
		indices: Vec::new(),
		program: None
	};

	logger::rust_info(&"Loading shaders to memory...");
    let vert_shader: &str = &(resources.get(&JsValue::from_str("vert_shader")).as_string().unwrap_or(String::from("bad_value")));
	logger::rust_super_verbose(&("Vertex Shader is: ".to_owned() + &vert_shader));
    let frag_shader: &str = &(resources.get(&JsValue::from_str("frag_shader")).as_string().unwrap_or(String::from("bad_value")));
    logger::rust_super_verbose(&("Fragment Shader is: ".to_owned() + &frag_shader));
	logger::rust_info(&"...shaders load to memory complete.");

	logger::rust_info(&"Compiling shaders...");
	let vert_shader = compile_shader(&frame, WebGl2RenderingContext::VERTEX_SHADER, vert_shader)?;
    let frag_shader = compile_shader(&frame, WebGl2RenderingContext::FRAGMENT_SHADER, frag_shader)?;
    logger::rust_info(&"...shaders compilation complete.");

	logger::rust_info(&"Linking shaders...");
	link_program(&mut frame, &vert_shader, &frag_shader)?;
    logger::rust_info(&"...shaders linking complete");
	frame.context.use_program(frame.program.as_ref());

	logger::rust_info(&"Loading scene to memory...");
	let scene: &str = &(resources.get(&JsValue::from_str("cube")).as_string().unwrap_or(String::from("bad_value")));
	logger::rust_super_verbose(&("...Scene is:".to_owned() + &scene));

	logger::rust_info(&"Loading textures to memory...");
    let mut textures: Vec<String> = Vec::new();
	textures.push(resources.get(&JsValue::from_str("texture")).as_string().unwrap_or(String::from("bad_value")));
	logger::rust_info(&"...textures load to memory complete.");


	logger::rust_verbose(&"Parsing scene...");
	let objset = match wavefront_obj::obj::parse(scene)
	{
		Ok(objset) => objset,
		Err(e) => panic!("{}", e)
	};
	logger::rust_verbose(&"...scene parsing complete.");

	logger::rust_info(&"...scene loading complete.");

	logger::rust_info(&"Buffering scene to GPU...");
	webgl::buffer_scene(&mut frame, &objset, &textures)?;
	logger::rust_info(&"...scene buffering complete.");

	// Set up depth test
	logger::rust_verbose(&"Configuring GPU depth testing...");
	webgl::enable_depthtest(&frame)?;
	logger::rust_verbose(&"...configuration complete.");
	
	frame.context.clear_color(0.0, 0.0, 0.0, 0.0);
	logger::rust_info(&"...webgl initialisation complete.");

	logger::rust_info(&"Initializing animation loop...");
	webgl::initialize_animation(frame);
	logger::rust_info(&"...animation loop initialisation complete.");

    return Ok(());
}

#[wasm_bindgen]
pub fn enable_rotate_x()
{
	let controller_values = get_control_flags();
	let mut tmp = controller_values.lock().unwrap();
	tmp.rotate_x = true;
}

#[wasm_bindgen]
pub fn disable_rotate_x()
{
	let controller_values = get_control_flags();
	let mut tmp = controller_values.lock().unwrap();
	tmp.rotate_x = false;
}

#[wasm_bindgen]
pub fn enable_rotate_y()
{
	let controller_values = get_control_flags();
	let mut tmp = controller_values.lock().unwrap();
	tmp.rotate_y = true;
}

#[wasm_bindgen]
pub fn disable_rotate_y()
{
	let controller_values = get_control_flags();
	let mut tmp = controller_values.lock().unwrap();
	tmp.rotate_y = false;
}

#[wasm_bindgen]
pub fn enable_rotate_z()
{
	let controller_values = get_control_flags();
	let mut tmp = controller_values.lock().unwrap();
	tmp.rotate_z = true;
}

#[wasm_bindgen]
pub fn disable_rotate_z()
{
	let controller_values = get_control_flags();
	let mut tmp = controller_values.lock().unwrap();
	tmp.rotate_z = false;
}