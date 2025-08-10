#![allow(dead_code)]

pub mod fisheye;
pub mod perspective;

use crate::renderer::objects::ray::{Matrix, Ray, Unit, Vector, Vector3};

pub trait Camera {
    fn gen_ray(&self, u: usize, v: usize) -> Ray;

    fn get_dimensions(&self) -> &Dimensions;
}

#[derive(Debug, Clone)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

