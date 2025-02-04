use wavefront_obj::obj::*;
use js_sys::*;
use std::io::Cursor;
use image::ImageReader;

pub fn get_vertex_indices(data: &wavefront_obj::obj::Object) -> Vec<u16> {
	let mut shapes_out: Vec<u16> = Vec::new();

	for n in 0..data.geometry[0].shapes.len()
	{
		let Primitive::Triangle(x, y, z) = data.geometry[0].shapes[n].primitive else { break };
		shapes_out.push(y.0 as u16);
		shapes_out.push(z.0 as u16);
		shapes_out.push(x.0 as u16);
	}

	log_vertex_indices(&shapes_out);	

	return shapes_out;
}

pub fn get_vertex_indices_2(data: &wavefront_obj::obj::Object) -> Vec<u16>
{
	let mut shapes_out: Vec<u16> = Vec::new();

	//Face 1
		//Triangle 1
		shapes_out.push(0);
		shapes_out.push(1);
		shapes_out.push(2);

		//Triangle 2
		shapes_out.push(3);
		shapes_out.push(4);
		shapes_out.push(5);

	//Face 2
		//Triangle 1
		shapes_out.push(6);
		shapes_out.push(7);
		shapes_out.push(8);

		//Triangle 2
		shapes_out.push(9);
		shapes_out.push(10);
		shapes_out.push(11);

	//Face 3
		//Triangle 1
		shapes_out.push(12);
		shapes_out.push(13);
		shapes_out.push(14);

		//Triangle 2
		shapes_out.push(15);
		shapes_out.push(16);
		shapes_out.push(17);

	//Face 4
		//Triangle 1
		shapes_out.push(18);
		shapes_out.push(19);
		shapes_out.push(20);

		//Triangle 2
		shapes_out.push(21);
		shapes_out.push(22);
		shapes_out.push(23);

	//Face 5
		//Triangle 1
		shapes_out.push(24);
		shapes_out.push(25);
		shapes_out.push(26);

		//Triangle 2
		shapes_out.push(27);
		shapes_out.push(28);
		shapes_out.push(29);

	//Face 6
		//Triangle 1
		shapes_out.push(30);
		shapes_out.push(31);
		shapes_out.push(32);

		//Triangle 2
		shapes_out.push(33);
		shapes_out.push(34);
		shapes_out.push(35);

	return shapes_out;
}

pub fn get_vertices(data: &wavefront_obj::obj::Object) -> Vec<f32> {
	let mut vertices_out: Vec<f32> = Vec::new();

	for n in 0..data.vertices.len()
	{
		vertices_out.push(data.vertices[n].x as f32);
		vertices_out.push(data.vertices[n].z as f32);
		vertices_out.push(data.vertices[n].y as f32);
	}

	return vertices_out;
}

pub fn get_texture_vertices(data: &wavefront_obj::obj::Object) -> Vec<f32> {
	let mut vertices_out: Vec<f32> = Vec::new();

	for n in 0..data.tex_vertices.len()
	{
		vertices_out.push(data.tex_vertices[n].u as f32);
		vertices_out.push(data.tex_vertices[n].v as f32);
		//vertices_out.push(data.tex_vertices[n].w as f32);
	}
	return vertices_out;
}

pub fn get_texture_indices(data: &wavefront_obj::obj::Object) -> Vec<u16>
{
	let mut shapes_out: Vec<u16> = Vec::new();

	for n in 0..data.geometry[0].shapes.len()
	{
		let Primitive::Triangle(x, y, z) = data.geometry[0].shapes[n].primitive else { break };
		match y.1 
		{
			Some(y) => shapes_out.push(y as u16),
			None => break
		};
		match z.1 
		{
			Some(z) => shapes_out.push(z as u16),
			None => break
		};
		match x.1 
		{
			Some(x) => shapes_out.push(x as u16),
			None => break
		};
	}

	return shapes_out;
}

pub fn merge_vertex_and_texture_positions(vertex_positions: &Vec<f32>, vertex_indices: &Vec<u16>, texture_positions: &Vec<f32>, texture_indices: &Vec<u16>) -> Vec<f32>
{
	let mut merged_vertex_and_texture_positions: Vec<f32> = Vec::new();

	for n in 0..vertex_indices.len()
	{
		let vertex_index = vertex_indices[n];
		merged_vertex_and_texture_positions.push(vertex_positions[(vertex_indices[n] as usize * 3) + 0]);
		merged_vertex_and_texture_positions.push(vertex_positions[(vertex_indices[n] as usize * 3) + 1]);
		merged_vertex_and_texture_positions.push(vertex_positions[(vertex_indices[n] as usize * 3) + 2]);
		merged_vertex_and_texture_positions.push(texture_positions[(texture_indices[n] as usize * 2) + 0]);
		merged_vertex_and_texture_positions.push(1.0 - (texture_positions[(texture_indices[n] as usize * 2) + 1]));
	}

	log_merged_vertex_and_texture_positions(&merged_vertex_and_texture_positions);

	return merged_vertex_and_texture_positions;
}

/*
pub fn get_texture_coords (data: &wavefront_obj::obj::Object) -> Vec<f32>
{
	let mut vertices_out: Vec<f32> = Vec::new();

	for n in 0..data.tex_vertices.len()
	{
		vertices_out.push(data.tex_vertices[n].u as f32);
		vertices_out.push(data.tex_vertices[n].v as f32);
		//vertices_out.push(data.tex_vertices[n].w as f32);
	}

	for n in 0..data.geometry[0].shapes.len()
	{
		let Primitive::Triangle(x, y, z) = data.geometry[0].shapes[n].primitive else { break };
		match x.1 
		{
			Some(x) => vertices_out.push(x as f32),
			None => break
		};
		match y.1 
		{
			Some(y) => vertices_out.push(y as f32),
			None => break
		};
		match z.1 
		{
			Some(z) => vertices_out.push(z as f32),
			None => break
		};
	}

	return vertices_out;
}
*/

// Not a real function. JUst testing as the real function for unknown reasons results in missailigned textures despite everything looking like it should align.
// This function returns hardcoded values that I can freely change
pub fn get_texture_coords_3 (model: &wavefront_obj::obj::Object) -> Vec<f32>
{
	let mut out: Vec<f32> = Vec::new();

	/*
	for n in 0..6
	{
		//Triangle 1
		out.push(0.75);
		out.push(0.5);
		out.push(0.75);
		out.push(0.25);
		out.push(0.5);
		out.push(0.25);

		//Triangle 2
		out.push(0.75);
		out.push(0.5);
		out.push(0.5);
		out.push(0.25);
		out.push(0.5);
		out.push(0.5);
	}
	*/

	//Push face 1
		//Triangle 1
		out.push(0.75);
		out.push(0.5);
		out.push(0.75);
		out.push(0.25);
		out.push(0.5);
		out.push(0.25);

		//Triangle 2
		out.push(0.75);
		out.push(0.5);
		out.push(0.5);
		out.push(0.25);
		out.push(0.5);
		out.push(0.5);

	//Push face 2
		//Triangle 1
		out.push(0.75);
		out.push(0.5);
		out.push(0.75);
		out.push(0.25);
		out.push(0.5);
		out.push(0.25);

		//Triangle 2
		out.push(0.75);
		out.push(0.5);
		out.push(0.5);
		out.push(0.25);
		out.push(0.5);
		out.push(0.5);

	//Push face 3
		//Triangle 1
		out.push(0.75);
		out.push(0.5);
		out.push(0.75);
		out.push(0.25);
		out.push(0.5);
		out.push(0.25);

		//Triangle 2
		out.push(0.75);
		out.push(0.5);
		out.push(0.5);
		out.push(0.25);
		out.push(0.5);
		out.push(0.5);

	//Push face 4
		//Triangle 1
		out.push(0.75);
		out.push(0.5);
		out.push(0.75);
		out.push(0.25);
		out.push(0.5);
		out.push(0.25);

		//Triangle 2
		out.push(0.75);
		out.push(0.5);
		out.push(0.5);
		out.push(0.25);
		out.push(0.5);
		out.push(0.5);

	//Push face 5
		//Triangle 1
		out.push(0.75);
		out.push(0.5);
		out.push(0.75);
		out.push(0.25);
		out.push(0.5);
		out.push(0.25);

		//Triangle 2
		out.push(0.75);
		out.push(0.5);
		out.push(0.5);
		out.push(0.25);
		out.push(0.5);
		out.push(0.5);

	//Push face 6
		//Triangle 1
		out.push(0.75);
		out.push(0.5);
		out.push(0.75);
		out.push(0.25);
		out.push(0.5);
		out.push(0.25);

		//Triangle 2
		out.push(0.75);
		out.push(0.5);
		out.push(0.5);
		out.push(0.25);
		out.push(0.5);
		out.push(0.5);

	return out
}

pub fn get_texture_coords_2 (model: &wavefront_obj::obj::Object) -> Vec<f32>
{
	let mut tex_coords: Vec<f32> = Vec::new();

	let mut tex_coords_out: Vec<f32> = Vec::new();

	//Referenced from faces
	for n in 0..model.tex_vertices.len()
	{
		//Note that uv works from bottom left in file but top left in model. Adjusted accordingly.
		tex_coords.push(model.tex_vertices[n].u as f32);
		tex_coords.push(1.0 - (model.tex_vertices[n].v) as f32);
		//vertices_out.push(data.tex_vertices[n].w as f32);
	}

	for n in 0..model.geometry[0].shapes.len()
	{
		let Primitive::Triangle(x, y, z) = model.geometry[0].shapes[n].primitive else { break };
		match y.1 
		{
			Some(y) => 
			{
				tex_coords_out.push(tex_coords[y*2] as f32);
				tex_coords_out.push(tex_coords[(y*2) + 1] as f32);
			},
			None => break
		};
		match z.1 
		{
			Some(z) => 
			{
				tex_coords_out.push(tex_coords[z*2] as f32);
				tex_coords_out.push(tex_coords[(z*2) + 1] as f32);
			},
			None => break
		};
		match x.1 
		{
			Some(x) => 
			{
				tex_coords_out.push(tex_coords[x*2] as f32);
				tex_coords_out.push(tex_coords[(x*2) + 1] as f32);
			},
			None => break
		};
	}

	log_texture_positions(&tex_coords_out);	

	return tex_coords_out;
}

pub fn create_image_as_uint8_array(base64_png: &str) -> Result<Uint8Array, String> 
{
	// Convert base64 to a binary array
	let bytes = base64::decode(base64_png).map_err(|_| "Failed to decode base64")?;

	let img1 = match ImageReader::new(Cursor::new(bytes)).with_guessed_format()
	{
		Ok(img1) => img1,
		Err(e) => return Err(e.to_string())
	};

	let img2 = match img1.decode()
	{
		Ok(img2) => img2,
		Err(e) => return Err(e.to_string())
	};

	let rgba_img = img2.to_rgba8();

	// Get image dimensions
    let (width, height) = rgba_img.dimensions();
	web_sys::console::log_1(&("Image Size: ".to_owned() + width.to_string().as_str() + " x " + height.to_string().as_str()).into());

	// Access raw pixel data
    let pixels = rgba_img.as_raw();

	// Create a Blob from binary data
    let array = js_sys::Uint8Array::from(pixels.as_slice());

	//log_js_uint8_array(&array);

	return Ok(array);	
}

pub fn log_vertex_indices(vertex_indices: &Vec<u16>)
{
	web_sys::console::log_1(&("Loaded vertex indices are: ".to_owned()).into());
	for n in 0..vertex_indices.len()
	{
		if n % 3 == 0
		{
			web_sys::console::log_1
			(
				&(
					vertex_indices[n].to_string().as_str().to_owned() + 
					" " + 
					vertex_indices[n + 1].to_string().as_str() +
					" " + 
					vertex_indices[n + 2].to_string().as_str()	
				).into()
			);
		}
	}
}

pub fn log_vertex_index_positions(vertex_indices: &Vec<u16>, vertex_positions: &Vec<f32>)
{
	let mut vertex_positions_translated: Vec<f32> = Vec::new();

	for n in 0..vertex_indices.len()
	{
		vertex_positions_translated.push(vertex_positions[vertex_indices[n] as usize * 3] as f32);
		vertex_positions_translated.push(vertex_positions[(vertex_indices[n] as usize * 3) + 1] as f32);
		vertex_positions_translated.push(vertex_positions[(vertex_indices[n] as usize * 3) + 2] as f32);
	}

	web_sys::console::log_1(&("Loaded vertex positions are: ".to_owned()).into());
	for n in 0..vertex_positions_translated.len()
	{
		if n % 9 == 0
		{
			web_sys::console::log_1
			(
				&(
					vertex_positions_translated[n].to_string().as_str().to_owned() + " " + vertex_positions_translated[n + 1].to_string().as_str() + " " + vertex_positions_translated[n + 2].to_string().as_str() +
					", " + 
					vertex_positions_translated[n + 3].to_string().as_str() + " " + vertex_positions_translated[n + 4].to_string().as_str() + " " + vertex_positions_translated[n + 5].to_string().as_str() +
					", " + 
					vertex_positions_translated[n + 6].to_string().as_str() + " " + vertex_positions_translated[n + 7].to_string().as_str() + " " + vertex_positions_translated[n + 8].to_string().as_str()	
				).into()
			);
		}
	}
}

pub fn log_merged_vertex_and_texture_positions(coords: &Vec<f32>)
{
	web_sys::console::log_1(&("Merged vertex & texture positions buffer is : ".to_owned()).into());
	for n in 0..coords.len()
	{
		if n % 5 == 0
		{
			web_sys::console::log_1
			(
				&(
					coords[n].to_string().as_str().to_owned() + ", " + coords[n + 1].to_string().as_str() + ", " + coords[n + 2].to_string().as_str() + 
					" - " + 
					coords[n + 3].to_string().as_str() + " " + coords[n + 4].to_string().as_str()
				).into()
			);
		}
	}
}

pub fn log_texture_positions(tex_coords_out: &Vec<f32>)
{
	web_sys::console::log_1(&("Loaded texure coordinates are: ".to_owned()).into());
	for n in 0..tex_coords_out.len()
	{
		if n % 6 == 0
		{
			web_sys::console::log_1
			(
				&(
					tex_coords_out[n].to_string().as_str().to_owned() + " " + tex_coords_out[n + 1].to_string().as_str() + 
					", " + 
					tex_coords_out[n + 2].to_string().as_str() + " " + tex_coords_out[n + 3].to_string().as_str() +
					", " + 
					tex_coords_out[n + 4].to_string().as_str() + " " + tex_coords_out[n + 5].to_string().as_str()	
				).into()
			);
		}
	}
}

pub fn log_js_uint8_array(array: &js_sys::Uint8Array)
{
	web_sys::console::log_1(&("Loaded texure coordinates are: ".to_owned()).into());
	web_sys::console::log_1(&(array.to_string().as_string()).into());
}
