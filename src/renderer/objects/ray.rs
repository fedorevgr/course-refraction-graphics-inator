use image::Rgb;
use nalgebra::{Matrix4, Vector4};
use nalgebra::Unit as U;


pub type Vector = Vector4<f64>;
pub type Unit = U<Vector>;

pub type Matrix = Matrix4<f64>;

pub type RGB = Rgb<u8>;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Unit,
    
    pub color: RGB,
}

impl Ray {
    pub fn new(origin: Vector, direction: Unit) -> Ray {
        Ray { origin, direction, color: Rgb([0, 0, 0]) }
    }
}