use wavefront_obj::obj::*;

pub fn parse_obj(data: &str) -> ObjSet {
    let obj_result = wavefront_obj::obj::parse(data);
    let obj: ObjSet;

    match obj_result {
        Ok(data) => obj = data,
        Err(err) => panic!(err)
    }

    web_sys::console::log_1(&("Loaded ".to_owned() + &obj.objects.len().to_string() + " model").into());
    return obj;
}

pub fn get_vertices(data: &ObjSet) -> Vec<f32> {
    let vertices_out: Vec<f32> = Vec::new();

    let vertices_obj: &Vec<Vertex> = &data.objects[0].vertices;


    //let vertices: [f32; 9] = [
    //    -0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0
    //];

    return vertices_out; 
}