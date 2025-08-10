use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::material::Material;
use crate::renderer::objects::model::Model;
use crate::renderer::objects::ray::{Ray, Unit, Vector};

#[derive(Debug, Clone)]
pub struct SphereModel {
    center: Vector,
    radius_sq: f64,
    material: Material,
}

impl SphereModel {
    pub fn new(center: Vector, radius: f64, material: Material) -> SphereModel {
        SphereModel {
            center,
            radius_sq: radius * radius,
            material,
        }
    }
}

impl Model for SphereModel {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let b = 2. * ray.direction.dot(&(ray.origin - self.center));
        let c = (self.center - ray.origin).magnitude_squared() - self.radius_sq;

        let d = b * b - 4. * c;
        if d < 0. {
            None
        } else {
            let t = (-b - d.sqrt()) / 2.;
            let hit_pos = ray.origin + ray.direction.scale(t);
            Some(Hit::new(
                t,
                hit_pos,
                &self.material,
                Unit::new_normalize(hit_pos - self.center),
            ))
        }
    }
}
