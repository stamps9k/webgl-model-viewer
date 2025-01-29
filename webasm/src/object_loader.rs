use wavefront_obj::obj::*;

pub fn get_vertex_indices(data: &Object) -> Vec<u16> {
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

pub fn get_vertices(data: &Object) -> Vec<f32> {
	let mut vertices_out: Vec<f32> = Vec::new();

	for n in 0..data.vertices.len()
	{
		vertices_out.push(data.vertices[n].x as f32);
		vertices_out.push(data.vertices[n].y as f32);
		vertices_out.push(data.vertices[n].z as f32);
	}
	return vertices_out;
}
