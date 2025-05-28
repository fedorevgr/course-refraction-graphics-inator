use nalgebra::{Matrix3, Vector3};

pub type Vector = Vector3<f64>;
pub type Matrix = Matrix3<f64>;

pub struct Ray {
    origin: Vector,
    direction: Vector
}

impl Ray {
    fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }
}