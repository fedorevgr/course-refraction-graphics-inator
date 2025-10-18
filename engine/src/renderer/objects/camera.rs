#![allow(dead_code)]

pub mod fisheye;
pub mod perspective;

use serde::{Deserialize, Serialize};
use crate::renderer::objects::ray::{Ray, Vector};

pub trait Camera {
    fn gen_ray(&self, u: usize, v: usize) -> Ray;

    fn get_dimensions(&self) -> &Dimensions;
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

