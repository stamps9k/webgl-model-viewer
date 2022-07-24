mod utils;
mod shaders;
mod object_loader;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;
use shaders::*;
use std::collections::*;
use js_sys::Map;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn initialize_web_gl(resources: Map) -> Result<(), JsValue> {
    web_sys::console::log_1(&"Initialising...".into());

    let vert_shader: &str = &(resources.get(&JsValue::from_str("vert_shader")).as_string().unwrap_or(String::from("bad_value")));
    let frag_shader: &str = &(resources.get(&JsValue::from_str("frag_shader")).as_string().unwrap_or(String::from("bad_value")));
    let obj = object_loader::parse_obj(&(resources.get(&JsValue::from_str("teapot")).as_string().unwrap_or(String::from("bad_value"))));

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("glCanvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap();
    let context = canvas.get_context("webgl2")?.unwrap().dyn_into::<web_sys::WebGl2RenderingContext>()?;

    web_sys::console::log_1(&("Vertex Shader is: ".to_owned() + &vert_shader).into());
    web_sys::console::log_1(&("Fragment Shader is: ".to_owned() + &frag_shader).into());

    let vert_shader = compile_shader(&context, WebGl2RenderingContext::VERTEX_SHADER, vert_shader)?;
    let frag_shader = compile_shader(&context, WebGl2RenderingContext::FRAGMENT_SHADER, frag_shader)?;

    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    //let mut vertices: Vec<f32> = object_loader::get_vertices(&obj);

    
    let mut vertices: Vec<f32> = Vec::new();
    
    for n in 0..9 {
        if (n == 0) {
            vertices.push(-0.7);
        } else if (n==1) {
            vertices.push(-0.7);
        } else if (n==3) {
            vertices.push(0.7);
        } else if (n==4) {
            vertices.push(-0.7);
        } else if (n==7) {
            vertices.push(0.7);
        } else {
            vertices.push(0.0);
        }
    }

    let color: [f32; 4] = [
        1.0,1.0,0.0,1.0
    ];

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let position_index = context.get_attrib_location(&program, "position") as u32;
    web_sys::console::log_1(&("Position Index is: ".to_owned() +  &(position_index.to_string())).into());
    context.vertex_attrib_pointer_with_i32(position_index, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(position_index);

    let color_index = context.get_uniform_location(&program, "u_color");
    context.uniform4fv_with_f32_array(color_index.as_ref(), &color);


    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(
        WebGl2RenderingContext::TRIANGLES,
        0,
        (vertices.len() / 3) as i32,
    );

    web_sys::console::log_1(&vert_shader);

    Ok(())
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}
