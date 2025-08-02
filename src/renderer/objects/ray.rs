use nalgebra::{Matrix4, Vector4};
use nalgebra::Vector3 as V3;
use nalgebra::Unit as U;

pub type Vector = Vector4<f64>;
pub type Vector3 = V3<f64>;
pub type Unit = U<Vector>;
pub type Unit3 = U<Vector3>;

pub type Matrix = Matrix4<f64>;

pub type Rgb = V3<u8>;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Unit,
    
    pub color: Rgb,
}

impl Ray {
    pub fn new(origin: Vector, direction: Unit) -> Ray {
        Ray { origin, direction, color: Rgb::from([0, 0, 0]) }
    }
}

