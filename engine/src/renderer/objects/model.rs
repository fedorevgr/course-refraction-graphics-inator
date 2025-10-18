

pub mod sphere;
pub mod triangle;
pub mod torus;

use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::ray::{Ray, Vector};

pub trait Model {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}


#[allow(dead_code)]
pub trait Rotate {
    fn set_rotation(&mut self, pitch: f64, yaw: f64, roll: f64);
    fn rotate_by(&mut self, pitch: f64, yaw: f64, roll: f64);
}

#[allow(dead_code)]
pub trait Move {
    fn set_position(&mut self, position: Vector);
    fn reposition_by(&mut self, pos: &Vector);
}

#[allow(dead_code)]
pub trait Scale {
    
}