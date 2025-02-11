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
use std::cell::RefCell;
use std::f64::consts::*;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use math::mean;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
static CONTROL_FLAGS: OnceLock<Arc<Mutex<ControllerValues>>> = OnceLock::new();

fn get_control_flags() -> Arc<Mutex<ControllerValues>> 
{
    CONTROL_FLAGS
        .get_or_init(|| Arc::new(Mutex::new(ControllerValues::new())))
        .clone()
}

fn window() -> web_sys::Window 
{
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) 
{
	window()
    	.request_animation_frame(f.as_ref().unchecked_ref())
    	.expect("should register `requestAnimationFrame` OK");
}

fn initialize_animation(mut frame: WebGl2Frame) 
{
	//Closure variables
	let f = Rc::new(RefCell::new(None));
    let g = f.clone();

	//Movement variables
	let mut rotating: bool = true;
	let controller_values = get_control_flags();

	//FPS calculator variables
	let mut base: f64 = get_current_time();
	let mut frames_delta: [f64; 10] = [0.0; 10];

	let tmp = frame.program.as_mut().unwrap().clone();

	let mut camera_matrix = Mat4::identity();
	camera_matrix.translate(&[0.0 as f32, 0.0 as f32, -10.0 as f32]);

	let mut rotation_angle: f32 = 0.0;
    let mut i: f32 = 0.0;
    *g.borrow_mut() = Some(Closure::new(move || {			
		//FPS Caclulator
		let now = get_current_time();
		match i as i32 % 10 
		{
			0 => frames_delta[0] = now - base,
			1 => frames_delta[1] = now - base,
			2 => frames_delta[2] = now - base,
			3 => frames_delta[3] = now - base,
			4 => frames_delta[4] = now - base,
			5 => frames_delta[5] = now - base,
			6 => frames_delta[6] = now - base,
			7 => frames_delta[7] = now - base,
			8 => frames_delta[8] = now - base,
			9 => 
			{
				frames_delta[9] = now - base;
				base = get_current_time();
				let fps: f64 = mean::arithmetic(&frames_delta);
				set_fps(fps);
			},
			_ => panic!("Don't know how you got here!")
		}

		camera_matrix = update_camera_position(&camera_matrix, &controller_values.lock().unwrap());

		//Pass worldspace transfomration to the GPU
		let position_index = frame.context.get_uniform_location(&tmp, "u_camera_matrix");
		frame.context.uniform_matrix4fv_with_f32_array(position_index.as_ref(), false, &camera_matrix);

		m4_pretty_print("Camera Matrix", &camera_matrix);
		
		
		draw(&frame.context, &frame.indices);

        // Set the body's text content to how many times this
        // requestAnimationFrame callback has fired.
        i += 1.0;

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

	request_animation_frame(g.borrow().as_ref().unwrap());
    
}

fn draw(context: &WebGl2RenderingContext, indices: &Vec<u16>) 
{
	logger::rust_super_super_verbose("Initiating draw call...");
	logger::rust_super_super_verbose(&("drawing ".to_owned() + indices.len().to_string().as_str() + " indices"));
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
	context.draw_elements_with_f64(WebGl2RenderingContext::TRIANGLES, indices.len() as i32, WebGl2RenderingContext::UNSIGNED_SHORT, 0.0);
	logger::rust_super_super_verbose("...draw call complete.");
}

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
	initialize_animation(frame);
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