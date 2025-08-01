use image::Rgb;
use nalgebra::{Matrix4, Vector4, Unit};


pub type Vector = Vector4<f64>;
pub type Matrix = Matrix4<f64>;

pub type RGB = Rgb<u8>;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Unit<Vector>,
    
    pub color: RGB,
}

impl Ray {
    pub fn new(origin: Vector, direction: Unit<Vector>) -> Ray {
        Ray { origin, direction, color: Rgb([0, 0, 0]) }
    }
}