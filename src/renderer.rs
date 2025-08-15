pub mod scene;
pub mod objects;
pub mod implementations;

use objects::ray::{Ray, RgbIntensity};

pub trait Renderer {
    fn cast(&self, ray: &Ray) -> RgbIntensity;
}
