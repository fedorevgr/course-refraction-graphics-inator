use std::fmt::Debug;

use crate::renderer::objects::ray::{Ray, Vector};

pub mod material {
    use crate::renderer::objects::ray::RGB;
    
    #[derive(Debug, Clone)]
    pub struct Material {
        pub absorption: RGB, // main color
        pub emissivity: RGB, // shine color

        pub roughness: f64, // reflect note: RGB
        pub transmittance: f64  // refract note: RGB
    }

    impl Material {
        // todo
    }

    impl Default for Material {
        fn default() -> Self {
            Material{
                absorption: RGB::from([128, 128, 128]),
                emissivity: RGB::from([0, 0, 0]),
                roughness: 1.,
                transmittance: 0.
            }
        }
    }
}

use material::Material;

#[derive(Debug)]
pub struct Hit {
    hit: bool,
    ray: Vec<Ray>,
}

pub trait Model
where Self: Clone + Debug
{
    fn hit(&self, ray: &Ray) -> Hit;
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub normal: Vector,
    pub idx: [usize; 3],
}

#[derive(Debug, Clone)]
pub struct TriangleModel {
    points: Vec<Vector>,
    triangles: Vec<Triangle>,
}

impl TriangleModel {
    // todo: new
}

impl Model for TriangleModel {
    fn hit(&self, ray: &Ray) -> Hit {
        todo!()
    }
}


#[derive(Debug, Clone)]
pub struct SphereModel {
    center: Vector,
    radius: f64,
    material: Material
}

impl SphereModel {
    pub fn new(center: Vector, radius: f64, material: Material) -> SphereModel {
        SphereModel {center, radius, material}
    }
}

impl Model for SphereModel {
    fn hit(&self, ray: &Ray) -> Hit {
        todo!()
    }
}