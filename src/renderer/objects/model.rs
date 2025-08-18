

pub mod sphere;
pub mod triangle;
pub mod torus;

use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::ray::{Ray, Vector};

pub trait Model {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

pub trait Transform {
    fn set_position(&mut self, position: Vector);
    fn set_rotation(&mut self, pitch: f64, yaw: f64, roll: f64);

    fn reposition_by(&mut self, pos: &Vector);

    fn rotate_by(&mut self, pitch: f64, yaw: f64, roll: f64);
}