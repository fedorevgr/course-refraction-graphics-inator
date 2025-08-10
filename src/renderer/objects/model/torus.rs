use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::material::Material;
use crate::renderer::objects::model::Model;
use crate::renderer::objects::ray::{Ray, Unit, Vector};

pub struct TorusModel {
    pub r: f64,
    pub k: f64,
    pub material: Material,
}

impl Model for TorusModel {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let step = 0.05;
        let dir = ray.direction.normalize();
        for i in 0..60 {
            let t = step * i as f64;
            let p = ray.origin + dir * t;

            if (p.magnitude_squared() + self.r - self.k).powi(2)
                < 4. * self.r * (p.x.powi(2) + p.y.powi(2))
            {
                return Some(Hit::new(
                    t,
                    p,
                    &self.material,
                    Unit::new_normalize(p - (p - Vector::new(0., 0., p.z, 0.))),
                ));
            }
        }
        None
    }
}
