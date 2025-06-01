use image::Rgb;
use crate::renderer::ray::Vector;
use super::ray::Ray;


#[derive(Debug)]
pub struct Triangle {
    pub normal: Vector,
    pub idx: [usize; 3],
}

// #[derive(Debug)]
// pub struct Model {
//     points: Vec<Vector>,
//     triangles: Vec<Triangle>,
// }
// 
// impl Model {
//     pub fn new(points: Vec<Vector>, triangles: Vec<Triangle>) -> Model {
//         Model { points, triangles }
//     }
//     
// }

#[derive(Debug)]
pub struct Material {
    pub color: Rgb<u8>,
    pub emissive: Rgb<u8>,
    pub roughness: f64
}

#[derive(Debug)]
pub struct Hit {
    hit: bool,
    ray: Vec<Ray>,
}


#[derive(Debug)]
pub struct Model {
    center: Vector,
    radius: f64,
    material: Material
}

impl Model {
    pub fn new(center: Vector, radius: f64, material: Material) -> Model {
        Model{center, radius, material}
    }
    
    
}