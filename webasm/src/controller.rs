use std::f64::consts::*;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use webgl_matrix::*;

#[derive(Clone)]
pub struct ControllerValues
{
    pub rotate_x: bool,
    pub rotate_y: bool,
    pub rotate_z: bool,
}

impl ControllerValues
{
    pub fn new() -> Self 
    {
        Self 
        {
            rotate_x: false,
            rotate_y: false,
            rotate_z: false
        }
    }
}

pub fn update_camera_position(camera_matrix: &Mat4, controller_values: &ControllerValues) -> Mat4
{
    let mut out = camera_matrix.clone();

    let rotation_angle: f32 = (PI / 180.0) as f32;

    if controller_values.rotate_x
    {
        let rotation_axis: [f32; 3] = [1.0, 0.0, 0.0]; 
        out.rotate(rotation_angle, &rotation_axis);
    } 
    if controller_values.rotate_y
    {
        let rotation_axis: [f32; 3] = [0.0, 1.0, 0.0]; 
        out.rotate(rotation_angle, &rotation_axis);
    } 
    if controller_values.rotate_z
    {
        let rotation_axis: [f32; 3] = [0.0, 0.0, 1.0]; 
        out.rotate(rotation_angle, &rotation_axis);
    }

    return out;
}