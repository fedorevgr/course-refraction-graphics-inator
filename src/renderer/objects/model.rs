

pub mod sphere;
pub mod triangle;
pub mod torus;

use std::error::Error;
use std::fmt::Debug;

use std::fs::OpenOptions;
use std::path::Path;

use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::material::Material;
use crate::renderer::objects::ray::{Ray, Unit, Vector, Vector3};

pub trait Model {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}
