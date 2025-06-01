
use crate::renderer::objects::ray::{Ray, RGB, Vector};


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
    pub absorption: RGB, // main color
    pub emissivity: RGB, // shine color

    pub roughness: f64, // reflect note: RGB
    pub transmittance: f64  // refract note: RGB
}

impl Material {
    pub fn Default() -> Self {
        Material{ 
            absorption: RGB::from([128, 128, 128]), 
            emissivity: RGB::from([0, 0, 0]), 
            roughness: 1., 
            transmittance: 0. 
        }
    }
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
    
    pub fn hit(&self, ray: &Ray) -> Hit {
        todo!()
    }
}