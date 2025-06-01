use image::Rgb;
use nalgebra::{Matrix4, Vector4, Unit};


pub type Vector = Vector4<f64>;
pub type Matrix = Matrix4<f64>;

#[derive(Debug)]
pub struct Ray {
    pub(crate) origin: Vector,
    pub(crate) direction: Unit<Vector>,
    
    pub color: Rgb<u8>,
}

impl Ray {
    pub fn new(origin: Vector, direction: Unit<Vector>) -> Ray {
        Ray { origin, direction, color: Rgb([0, 0, 0]) }
    }
}