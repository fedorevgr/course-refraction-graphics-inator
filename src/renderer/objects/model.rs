

pub mod sphere;
pub mod triangle;
pub mod torus;

use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::ray::Ray;

pub trait Model {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}
