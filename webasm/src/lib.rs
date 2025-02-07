mod shaders;
mod utils;
mod object_loader;
mod logger;
mod webgl;

use crate::shaders::*;
use crate::logger::*;
use crate::webgl::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;
use webgl_matrix::*;
use js_sys::Map;
use std::cell::RefCell;
use std::rc::Rc;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/*#[wasm_bindgen]
extern {
    fn alert(s: &str);
}*/

fn window() -> web_sys::Window {
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
	let f = Rc::new(RefCell::new(None));
    let g = f.clone();

	let projection_matrix = Mat4::create_perspective(1.0471975511965976, 0.8260869565217391, 1.0, 2000.0);
	let mut camera_matrix = Mat4::identity();
	let camera_translation: [f32; 3] = [0.0, 0.0, -10.0];
	camera_matrix.translate(&camera_translation);

	let tmp = frame.program.as_mut().unwrap().clone();

    let mut i: f32 = 0.0;
    *g.borrow_mut() = Some(Closure::new(move || {
        if i > 360.0 {
					// Drop our handle to this closure so that it will get cleaned
          // up once we return.
          let _ = f.borrow_mut().take();
          return;
        }

				let rotation_axis: [f32; 3] = [1.0, 1.0, 0.0];
				camera_matrix.rotate(0.01745329, &rotation_axis);
				let mut view_matrix = camera_matrix.clone();
				let view_projection_matrix = view_matrix.mul(&projection_matrix);

				let position_index = frame.context.get_uniform_location(&tmp, "u_matrix");
				frame.context.uniform_matrix4fv_with_f32_array(position_index.as_ref(), false, view_projection_matrix);
				
				m4_pretty_print("Projection Matrix", &projection_matrix);
				m4_pretty_print("View Matrix", &camera_matrix);
				m4_pretty_print("View Projection Matrix", &view_projection_matrix);
				
				draw(&frame.context, &frame.indices);

        // Set the body's text content to how many times this
        // requestAnimationFrame callback has fired.
        i += 1.0;

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

		request_animation_frame(g.borrow().as_ref().unwrap());
    
}

fn draw(context: &WebGl2RenderingContext, indices: &Vec<u16>) {
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