use std::fmt::Debug;
use image::Rgb;
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

#[derive(Debug, Clone)]
pub struct Hit<'a> {
    pub pos: Vector,
    pub material: &'a Material,
    pub normal: Vector,
}

impl<'a> Hit<'a> {
    pub fn new(
        pos: Vector,
        material: &'a Material,
        normal: Vector,
    ) -> Self {
        Hit { pos, material, normal }
    }
}

pub trait Model
where Self: Clone + Debug
{
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub normal: Vector,
    pub idx: [usize; 3],
}

impl Triangle {
    pub fn point_in(&self, point: &Vector) -> bool {
        true // todo
    }

    pub fn intersect(&self, ray: &Ray) -> f64 {
        0. // todo
    }
}

#[derive(Debug, Clone)]
pub struct TriangleModel {
    points: Vec<Vector>,
    triangles: Vec<Triangle>,
}

impl TriangleModel {

}

impl Model for TriangleModel {
    fn hit(&self, ray: &Ray) -> Option<Hit> {

        for triangle in &self.triangles {
            let t = triangle.intersect(ray);
            if t >= 0. {
                let hit_pos = ray.origin + ray.direction.scale(t);

                if triangle.point_in(&hit_pos) {
                    
                }
            }
        }

        todo!()
    }
}


#[derive(Debug, Clone)]
pub struct SphereModel {
    center: Vector,
    radius_sq: f64,
    material: Material
}

impl SphereModel {
    pub fn new(center: Vector, radius: f64, material: Material) -> SphereModel {
        SphereModel {center, radius_sq: radius * radius, material}
    }
}

impl Model for SphereModel {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let b = 2. * ray.direction.dot(&(self.center - ray.origin));
        let c  = (self.center - ray.origin).magnitude() - self.radius_sq;

        let d = b * b - 4. * c;
        if d < 0. {
            None
        }
        else {
            let t = (-b - d.sqrt()) / 2.;
            let hit_pos = ray.origin + ray.direction.scale(t);
            Some(
                Hit::new(
                    hit_pos,
                    &self.material,
                hit_pos - self.center
                )
            )
        }
    }
}