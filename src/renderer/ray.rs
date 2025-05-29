use nalgebra::{Matrix4, Vector4, Unit};


pub type Vector = Vector4<f64>;
pub type Matrix = Matrix4<f64>;

pub struct Ray {
    origin: Vector,
    direction: Unit<Vector>
}

impl Ray {
    fn new(origin: Vector, direction: Unit<Vector>) -> Ray {
        Ray { origin, direction }
    }
}