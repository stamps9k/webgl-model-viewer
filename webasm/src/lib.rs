mod utils;
mod shaders;
mod object_loader;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;
use shaders::*;
use webgl_matrix::*;
use js_sys::Map;
use std::cell::RefCell;
use std::rc::Rc;
use wavefront_obj::obj::*;
use rand::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

fn draw(context: &WebGl2RenderingContext, indices: &Vec<u16>) {
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
	context.draw_elements_with_f64(WebGl2RenderingContext::TRIANGLES, indices.len() as i32, WebGl2RenderingContext::UNSIGNED_SHORT, 0.0);
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
	window()
    .request_animation_frame(f.as_ref().unchecked_ref())
    .expect("should register `requestAnimationFrame` OK");
}

fn initialize_animation(context: WebGl2RenderingContext, program: web_sys::WebGlProgram, indices: Vec<u16>) {
	let f = Rc::new(RefCell::new(None));
    let g = f.clone();

	let projection_matrix = Mat4::create_perspective(1.0471975511965976, 0.8260869565217391, 1.0, 2000.0);
	let mut camera_matrix = Mat4::identity();
	let camera_translation: [f32; 3] = [0.0, 0.0, -10.0];
	camera_matrix.translate(&camera_translation);

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

				let position_index = context.get_uniform_location(&program, "u_matrix");
				context.uniform_matrix4fv_with_f32_array(position_index.as_ref(), false, view_projection_matrix);
				
				//m4_pretty_print("Projection Matrix", &projection_matrix);
				//m4_pretty_print("View Matrix", &camera_matrix);
				//m4_pretty_print("View Projection Matrix", &view_projection_matrix);
				
				draw(&context, &indices);

        // Set the body's text content to how many times this
        // requestAnimationFrame callback has fired.
        i += 1.0;

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

		request_animation_frame(g.borrow().as_ref().unwrap());
    
}

#[wasm_bindgen]
pub fn initialize_web_gl(resources: Map) -> Result<(), JsValue> {
    web_sys::console::log_1(&"Initialising check...".into());

    let vert_shader: &str = &(resources.get(&JsValue::from_str("vert_shader")).as_string().unwrap_or(String::from("bad_value")));
    let frag_shader: &str = &(resources.get(&JsValue::from_str("frag_shader")).as_string().unwrap_or(String::from("bad_value")));
    let texture: &str = &(resources.get(&JsValue::from_str("texture")).as_string().unwrap_or(String::from("bad_value")));
		let objset = match wavefront_obj::obj::parse(&(resources.get(&JsValue::from_str("cube")).as_string().unwrap_or(String::from("bad_value"))))
		{
			Ok(objset) => objset,
			Err(e) => panic!("{}", e)
		};

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("glCanvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap();
    let context = canvas.get_context("webgl2")?.unwrap().dyn_into::<web_sys::WebGl2RenderingContext>()?;

    web_sys::console::log_1(&("Vertex Shader is: ".to_owned() + &vert_shader).into());
    web_sys::console::log_1(&("Fragment Shader is: ".to_owned() + &frag_shader).into());
		web_sys::console::log_1(&("Texture is: ".to_owned() + &texture).into());


    let vert_shader = compile_shader(&context, WebGl2RenderingContext::VERTEX_SHADER, vert_shader)?;
    let frag_shader = compile_shader(&context, WebGl2RenderingContext::FRAGMENT_SHADER, frag_shader)?;

    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

	let mut drawable_indices: Vec<u16> = Vec::new();

		for n in 0..(&objset).objects.len()
		{
			//Ignore junk objects
			if (&objset).objects[n].vertices.len() != 0	
			{
				web_sys::console::log_1(&("Sending model to GPU...").into());
				drawable_indices = buffer_obj(&context, &program, &objset.objects[n], texture)?;
				web_sys::console::log_1(&("...model sent.").into());
			}
		}

		// Set up depth test
		context.enable(WebGl2RenderingContext::DEPTH_TEST);
		context.depth_func(WebGl2RenderingContext::LESS);	
		
		//context.clear_color(1.0, 1.0, 1.0, 1.0);
		context.clear_color(0.0, 0.0, 0.0, 0.0);

		initialize_animation(context, program, drawable_indices);

    Ok(())
}

pub fn m4_pretty_print(name: &str, matrix: &[f32; 16])
{
	web_sys::console::log_1
	(
		&(
			"Matrix is ".to_owned() + name + ":"
		).into()
	);
	web_sys::console::log_1
	(
		&(
			matrix[0].to_string().as_str().to_owned() + ", " +
			matrix[4].to_string().as_str() + ", " +
			matrix[8].to_string().as_str() + ", " +
			matrix[12].to_string().as_str() 
		).into()
	);
	web_sys::console::log_1
	(
		&(
			matrix[1].to_string().as_str().to_owned() + ", " +
			matrix[5].to_string().as_str() + ", " +
			matrix[9].to_string().as_str() + ", " +
			matrix[13].to_string().as_str() 
		).into()
	);
	web_sys::console::log_1
	(
		&(
			matrix[2].to_string().as_str().to_owned() + ", " +
			matrix[6].to_string().as_str() + ", " +
			matrix[10].to_string().as_str() + ", " +
			matrix[14].to_string().as_str() 
		).into()
	);
	web_sys::console::log_1
	(
		&(
			matrix[3].to_string().as_str().to_owned() + ", " +
			matrix[7].to_string().as_str() + ", " +
			matrix[11].to_string().as_str() + ", " +
			matrix[15].to_string().as_str() 
		).into()
	);

}

pub fn buffer_obj(context: &web_sys::WebGl2RenderingContext, program: &web_sys::WebGlProgram, obj: &Object, texture_b64: &str) -> Result<Vec<u16>, String>
{
	//Indices to be returned
	let return_vec: Vec<u16>;

	/*
	*
	*	Get all relevant infomation from the wavefront object
	*
	*/
	let vertex_positions: Vec<f32> = object_loader::get_vertex_positions(&obj);
	web_sys::console::log_1(&("Vertices only is size: ".to_owned() + vertex_positions.len().to_string().as_str()).into());
	let vertex_indices: Vec<u16> = object_loader::get_vertex_indices(&obj);
	web_sys::console::log_1(&("Vertex Indices is size: ".to_owned() + vertex_indices.len().to_string().as_str()).into());

	let texture_vertices: Vec<f32> = object_loader::get_texture_positions(&obj);
	web_sys::console::log_1(&("Texutre Vertices is size: ".to_owned() + texture_vertices.len().to_string().as_str()).into());
	let texture_indices: Vec<u16> = object_loader::get_texture_indices(&obj);
	web_sys::console::log_1(&("Texutre Indices is size: ".to_owned() + texture_indices.len().to_string().as_str()).into());
	
	/*
		Create the vertex array object	
	*/
	let vao = context.create_vertex_array();
	context.bind_vertex_array(vao.as_ref());


	if texture_vertices.len() > 0 
	{
		/*
			Manage model texture and vertices
		*/
			// First generate the texture and vertex info
			let merged_array: Vec<f32> = object_loader::merge_vertex_and_texture_positions(&vertex_positions, &vertex_indices, &texture_vertices, &texture_indices);

			// create the GPU buffer
			let vertex_and_texture_buffer = context.create_buffer().ok_or("failed to create a buffer for textures")?;
			context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_and_texture_buffer));


			//Put values into buffer
			web_sys::console::log_1(&("Starting to buffer vertex & texture locations... ".to_owned()).into());
			unsafe {
				let texture_coord_array = js_sys::Float32Array::view(&merged_array);
			
				context.buffer_data_with_array_buffer_view
				(
					WebGl2RenderingContext::ARRAY_BUFFER,
					&texture_coord_array,
					WebGl2RenderingContext::STATIC_DRAW
				);
				web_sys::console::log_1(&("...Texture indice fully buffered".to_owned()).into());
			}

			//Tell GPU how to extract vertex data from the buffer
			let position_attribute_location = context.get_attrib_location(&program, "a_position") as u32;
			context.vertex_attrib_pointer_with_i32
			(
				position_attribute_location, //index
				3, //size
				WebGl2RenderingContext::FLOAT, //data type
				false, //normalized
				20, //stride
				0 //offset
			);

			//Tell GPU how to extract texture data from the buffer
			let texture_attribute_location = context.get_attrib_location(&program, "a_texcoord") as u32;
			web_sys::console::log_1(&("Texture attribute location found as : ".to_owned() + texture_attribute_location.to_string().as_str()).into());
			context.vertex_attrib_pointer_with_i32
			(
				texture_attribute_location, //index
				2, //size
				WebGl2RenderingContext::FLOAT, //data type
				false, //normalized 
				20, //stride
				12 //offset
			);

			context.enable_vertex_attrib_array(position_attribute_location);
			context.enable_vertex_attrib_array(texture_attribute_location);
			
			//Buffer the texture image
			web_sys::console::log_1(&("Starting to buffer texture image... ".to_owned()).into());
			let texture = context.create_texture().ok_or("failed to create texture")?;
			context.active_texture(WebGl2RenderingContext::TEXTURE0);
			context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
			let image = object_loader::create_image_as_uint8_array(texture_b64)?;
			web_sys::console::log_1(&("Image on wasm side is size: ".to_owned() + image.length().to_string().as_str()).into());
			context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
			let _ = 
			match 
				context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_js_u8_array
				(
					WebGl2RenderingContext::TEXTURE_2D,
					0,
					WebGl2RenderingContext::RGBA8 as i32,
					320,
					320,
					0,
					WebGl2RenderingContext::RGBA, // format
					WebGl2RenderingContext::UNSIGNED_BYTE, // type
					Some(&image)
				)
			{
				Ok(result) => result,
				Err(_err) => panic!("failed to send image data to texture buffer.")
			};
			context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

		/*
		Manage Indices for model
		*/
		web_sys::console::log_1(&("Starting to buffer vertex indices... ".to_owned()).into());
		let vert_index = context.create_buffer().ok_or("failed to create buffer")?;
		context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&vert_index));

		unsafe {
			return_vec = (0..merged_array.len() as u16).collect();
			let converted_indices = js_sys::Uint16Array::view(&return_vec);
		
			context.buffer_data_with_array_buffer_view(
				WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
				&converted_indices,
				WebGl2RenderingContext::STATIC_DRAW,
			);
			web_sys::console::log_1(&("..Vertex indices fully buffered.".to_owned()).into());
		}

	} else {
		/*
			Manage Vertices for model
		
			Note that `Float32Array::view` is somewhat dangerous (hence the
		`unsafe`!). This is creating a raw view into our module's
			`WebAssembly.Memory` buffer, but if we allocate more pages for ourself
		(aka do a memory allocation in Rust) it'll cause the buffer to change,
		causing the `Float32Array` to be invalid.
			
		As a result, after `Float32Array::view` we have to be very careful not to
		do any memory allocations before it's dropped.
		*/
		unsafe 
		{
			web_sys::console::log_1(&("Starting to buffer vertex data... ".to_owned()).into());
					
			let vert_buffer = context.create_buffer().ok_or("failed to create buffer")?;
			context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vert_buffer));

			let position_attribute_location = context.get_attrib_location(&program, "a_position") as u32;
			context.vertex_attrib_pointer_with_i32(position_attribute_location, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);	
					
			let vert_array = js_sys::Float32Array::view(&vertex_positions);
			context.buffer_data_with_array_buffer_view
			(
				WebGl2RenderingContext::ARRAY_BUFFER,
				&vert_array,
				WebGl2RenderingContext::STATIC_DRAW,
			);

				web_sys::console::log_1(&("..Vertex data fully buffered.".to_owned()).into());

				context.enable_vertex_attrib_array(position_attribute_location);
		}

		/*
			Manage Colors for model
		*/
		unsafe {
			web_sys::console::log_1(&("Starting to buffer color data... ".to_owned()).into());
			let color_buffer = context.create_buffer().ok_or("failed to create buffer")?;
			context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));
    		context.vertex_attrib_pointer_with_i32(1, 4, WebGl2RenderingContext::FLOAT, false, 0, 0);
			context.enable_vertex_attrib_array(1);
	
			let mut rng = rand::rng();
	
			//Currently junk colors. Only care about matching vertex count in sample cube
			let mut colors: Vec<f32> = Vec::new();
			for n in 0..vertex_indices.len()
			{
				if n % 3 == 0
				{
					let c: f32 = rng.random_range(0.0..=1.0);
					colors.push(c);
					colors.push(c);
					colors.push(c);
				}

			}
			web_sys::console::log_1(&("Built color array size is: ".to_owned() + colors.len().to_string().as_str()).into());

			let color_array = js_sys::Float32Array::view(&colors);

			context.buffer_data_with_array_buffer_view
			(
				WebGl2RenderingContext::ARRAY_BUFFER,
				&color_array,
      			WebGl2RenderingContext::STATIC_DRAW,
			);
			web_sys::console::log_1(&("...color data fully buffered.".to_owned()).into());
		}

		/*
		Manage Indices for model
		*/
		web_sys::console::log_1(&("Starting to buffer vertex indices... ".to_owned()).into());
		let vert_index = context.create_buffer().ok_or("failed to create buffer")?;
		context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&vert_index));

		unsafe {
			return_vec = object_loader::get_vertex_indices(&obj);
			let converted_indices = js_sys::Uint16Array::view(&return_vec);
		
			context.buffer_data_with_array_buffer_view(
				WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
				&converted_indices,
				WebGl2RenderingContext::STATIC_DRAW,
			);
			web_sys::console::log_1(&("..Vertex indices fully buffered.".to_owned()).into());
		}

	}
	
	return Ok(return_vec.clone());

}
