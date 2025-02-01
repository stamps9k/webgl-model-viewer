use wavefront_obj::obj::*;
use js_sys::*;
use std::io::Cursor;
use image::ImageReader;

pub fn get_vertex_indices(data: &wavefront_obj::obj::Object) -> Vec<u16> {
	let mut shapes_out: Vec<u16> = Vec::new();

	for n in 0..data.geometry[0].shapes.len()
	{
		let Primitive::Triangle(x, y, z) = data.geometry[0].shapes[n].primitive else { break };
		shapes_out.push(x.0 as u16);
		shapes_out.push(y.0 as u16);
		shapes_out.push(z.0 as u16);
	}

	return shapes_out;
}

pub fn get_vertices(data: &wavefront_obj::obj::Object) -> Vec<f32> {
	let mut vertices_out: Vec<f32> = Vec::new();

	for n in 0..data.vertices.len()
	{
		vertices_out.push(data.vertices[n].x as f32);
		vertices_out.push(data.vertices[n].y as f32);
		vertices_out.push(data.vertices[n].z as f32);
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

	// Access raw pixel data
    let pixels = rgba_img.as_raw();

	// Create a Blob from binary data
    let array = js_sys::Uint8Array::from(pixels.as_slice());

	return Ok(array);	
}
