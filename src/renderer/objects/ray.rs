use nalgebra::{Matrix4, Vector4};
use nalgebra::Vector3 as V3;
use nalgebra::Unit as U;

pub type Vector = Vector4<f64>;
pub type Vector3 = V3<f64>;
pub type Unit = U<Vector>;

pub type Matrix = Matrix4<f64>;

pub type Rgb = V3<u8>;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Unit
}

impl Ray {
    pub fn new(origin: Vector, direction: Unit) -> Ray {
        Ray { origin, direction }
    }
}

