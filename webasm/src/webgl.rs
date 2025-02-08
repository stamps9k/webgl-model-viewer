use crate::logger;
use crate::object_loader;

use web_sys::WebGl2RenderingContext;
use web_sys::WebGlProgram;
use webgl_matrix::*;
use wavefront_obj::obj::ObjSet;
use wavefront_obj::obj::Object;
use rand::prelude::*;

pub struct WebGl2Frame
{
	pub context: WebGl2RenderingContext,
	pub indices: Vec<u16>,
	pub program: Option<WebGlProgram>
} 

pub fn buffer_scene(frame: &mut WebGl2Frame, objset: &ObjSet, textures: &Vec<String>) -> Result<(), String>
{
	logger::rust_info(&"Loading textures to memory...");
	logger::rust_super_verbose(&("Texture is: ".to_owned() + &textures[0]));
	logger::rust_info(&"... texture loading complete.");

    for n in 0..(&objset).objects.len()
	{
		//Ignore junk objects
		if (&objset).objects[n].vertices.len() != 0	
		{
			logger::rust_info(&("Buffering model ".to_owned() + n.to_string().as_str() + ": " + &objset.objects[n].name + "to GPU..."));
			logger::rust_info(&(textures.len().to_string().as_str()));
			//TODO Properly generate and pass in textures.
			buffer_obj(frame, &objset.objects[n], textures[0].clone())?;
			logger::rust_info(&"...model buffering complete.");
		}
	}

	set_projection(frame);

	return Ok(());
}

pub fn enable_depthtest(frame: &WebGl2Frame) -> Result<(), String>
{
    frame.context.enable(WebGl2RenderingContext::DEPTH_TEST);
	frame.context.depth_func(WebGl2RenderingContext::LESS);

    return Ok(());
}

pub fn buffer_obj(frame: &mut WebGl2Frame, obj: &Object, texture_b64: String) -> Result<(), String>
{
	/*
	*
	*	Get all relevant infomation from the wavefront object
	*
	*/
	let vertex_positions: Vec<f32> = object_loader::get_vertex_positions(&obj);
	logger::rust_verbose(&("Vertices only is size: ".to_owned() + vertex_positions.len().to_string().as_str()));
	let vertex_indices: Vec<u16> = object_loader::get_vertex_indices(&obj);
	logger::rust_verbose(&("Vertex Indices is size: ".to_owned() + vertex_indices.len().to_string().as_str()));

	let texture_vertices: Vec<f32> = object_loader::get_texture_positions(&obj);
	logger::rust_verbose(&("Texutre Vertices is size: ".to_owned() + texture_vertices.len().to_string().as_str()));
	let texture_indices: Vec<u16> = object_loader::get_texture_indices(&obj);
	logger::rust_verbose(&("Texutre Indices is size: ".to_owned() + texture_indices.len().to_string().as_str()));
	
	/*
		Create the vertex array object	
	*/
	logger::rust_verbose(&"Creating vertex array object...");
	let vao = frame.context.create_vertex_array();
	frame.context.bind_vertex_array(vao.as_ref());
	logger::rust_verbose(&"...vertex array object creation completed.");

	logger::rust_verbose(&("Object: ".to_owned() + obj.name.as_str() + "identified as textured model. Processing accordingly"));
	if texture_vertices.len() > 0 
	{
		/*
			Manage model texture and vertices
		*/
			// First generate the texture and vertex info
			logger::rust_verbose("Generating a list that has all unique combined vertex + texture positions...");
			let merged_array: Vec<f32> = object_loader::merge_vertex_and_texture_positions(&vertex_positions, &vertex_indices, &texture_vertices, &texture_indices);
			logger::rust_verbose("...combined vertex + texture positions list completed.");

			// create the GPU buffer
			logger::rust_verbose("Creating  GPU buffer for vertex and texture positions array...");
			let vertex_and_texture_buffer = frame.context.create_buffer().ok_or("failed to create a buffer for textures")?;
			frame.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_and_texture_buffer));
			logger::rust_verbose("...GPU buffer creation complete.");

			//Put values into buffer
			logger::rust_verbose(&("Starting to buffer vertex & texture locations... "));
			unsafe {
				let texture_coord_array = js_sys::Float32Array::view(&merged_array);
			
				frame.context.buffer_data_with_array_buffer_view
				(
					WebGl2RenderingContext::ARRAY_BUFFER,
					&texture_coord_array,
					WebGl2RenderingContext::STATIC_DRAW
				);
			}
			logger::rust_verbose(&("...buffering complete."));

			//Tell GPU how to extract vertex data from the buffer
			let position_attribute_location = frame.context.get_attrib_location(&frame.program.as_mut().unwrap(), "a_position") as u32;
			frame.context.vertex_attrib_pointer_with_i32
			(
				position_attribute_location, //index
				3, //size
				WebGl2RenderingContext::FLOAT, //data type
				false, //normalized
				20, //stride
				0 //offset
			);

			//Tell GPU how to extract texture data from the buffer
			let texture_attribute_location = frame.context.get_attrib_location(&frame.program.as_mut().unwrap(), "a_texcoord") as u32;
			frame.context.vertex_attrib_pointer_with_i32
			(
				texture_attribute_location, //index
				2, //size
				WebGl2RenderingContext::FLOAT, //data type
				false, //normalized 
				20, //stride
				12 //offset
			);

			frame.context.enable_vertex_attrib_array(position_attribute_location);
			frame.context.enable_vertex_attrib_array(texture_attribute_location);
			
			//Buffer the texture image
			logger::rust_verbose(&("Starting to buffer texture image... "));
			let texture = frame.context.create_texture().ok_or("failed to create texture")?;
			frame.context.active_texture(WebGl2RenderingContext::TEXTURE0);
			frame.context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
			let image = object_loader::create_image_as_uint8_array(texture_b64.as_str())?;
			frame.context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
			let _ = 
			match 
				frame.context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_js_u8_array
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
			frame.context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
			logger::rust_verbose(&("...texture buffering complete."));

		/*
		Manage Indices for model
		*/
		logger::rust_verbose(&("Starting to buffer vertex & texture position indices... "));
		let vert_index = frame.context.create_buffer().ok_or("failed to create buffer")?;
		frame.context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&vert_index));
		unsafe {
			frame.indices = (0..(merged_array.len() / 5) as u16).collect();
			let converted_indices = js_sys::Uint16Array::view(&frame.indices);
			logger::rust_super_verbose(&(converted_indices.to_string().as_string().unwrap()));
			frame.context.buffer_data_with_array_buffer_view(
				WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
				&converted_indices,
				WebGl2RenderingContext::STATIC_DRAW,
			);
		}
		logger::rust_verbose(&("...buffering complete."));
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
			logger::rust_verbose(&("Starting to buffer vertex data... "));
			let vert_buffer = frame.context.create_buffer().ok_or("failed to create buffer")?;
			frame.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vert_buffer));

			let position_attribute_location = frame.context.get_attrib_location(&frame.program.as_mut().unwrap(), "a_position") as u32;
			frame.context.vertex_attrib_pointer_with_i32(position_attribute_location, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);	
					
			let vert_array = js_sys::Float32Array::view(&vertex_positions);
			frame.context.buffer_data_with_array_buffer_view
			(
				WebGl2RenderingContext::ARRAY_BUFFER,
				&vert_array,
				WebGl2RenderingContext::STATIC_DRAW,
			);
			frame.context.enable_vertex_attrib_array(position_attribute_location);
			logger::rust_verbose(&("..Vertex data fully buffered."));
		}

		/*
			Manage Colors for model
		*/
		unsafe {
			logger::rust_verbose(&("Starting to buffer color data... "));
			let color_buffer = frame.context.create_buffer().ok_or("failed to create buffer")?;
			frame.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));
    		frame.context.vertex_attrib_pointer_with_i32(1, 4, WebGl2RenderingContext::FLOAT, false, 0, 0);
			frame.context.enable_vertex_attrib_array(1);
	
			//Currently junk colors. Only care about matching vertex count in sample cube
			let mut rng = rand::rng();
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
			let color_array = js_sys::Float32Array::view(&colors);

			frame.context.buffer_data_with_array_buffer_view
			(
				WebGl2RenderingContext::ARRAY_BUFFER,
				&color_array,
      			WebGl2RenderingContext::STATIC_DRAW,
			);
			logger::rust_verbose(&("...color data buffering complete."));
		}

		/*
		Manage Indices for model
		*/
		logger::rust_verbose(&("Starting to buffer vertex indices... "));
		let vert_index = frame.context.create_buffer().ok_or("failed to create buffer")?;
		frame.context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&vert_index));
		unsafe {	 
			frame.indices = object_loader::get_vertex_indices(&obj);
			let converted_indices = js_sys::Uint16Array::view(&frame.indices);
		
			frame.context.buffer_data_with_array_buffer_view(
				WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
				&converted_indices,
				WebGl2RenderingContext::STATIC_DRAW,
			);
		}
		logger::rust_verbose(&("..indice buffering complete."));

	}
	return Ok(());
}

/*
*
* Sets the projections matrix. Currently has no projection is hardcoded
* TODO let user customise
*
*/
pub fn set_projection(frame: &WebGl2Frame)
{
	let projection_matrix = Mat4::create_perspective(1.0471975511965976, 0.8260869565217391, 1.0, 2000.0);
	let position_index = frame.context.get_uniform_location(&frame.program.as_ref().unwrap(), "u_projection_matrix");
	frame.context.uniform_matrix4fv_with_f32_array(position_index.as_ref(), false, &projection_matrix);

	logger::m4_pretty_print("Projection Matrix", &projection_matrix);
}