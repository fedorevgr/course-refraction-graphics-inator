pub mod scene;
pub mod objects;
pub mod implementations;

use objects::ray::{Ray, Rgb};

pub trait Renderer {
    fn cast(&self, ray: &Ray) -> Rgb;
}
