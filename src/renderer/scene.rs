use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::model::{Model};
use crate::renderer::objects::ray::Ray;

#[derive(Clone, Debug)]
pub struct Scene<M: Model> {
    pub objects: Vec<M>
}

impl<M: Model> Scene<M> {
    pub fn new(objects: Vec<M>) -> Self {
        Scene { objects }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut closest_t = f64::INFINITY;
        let mut closest: Option<Hit> = None;

        self.objects.iter().for_each(|object| {
            match object.hit(ray) {
                None => {}
                Some(hit) => {
                    if 0.0000001 < hit.factor && hit.factor < closest_t {
                        closest_t = hit.factor;
                        closest = Some(hit);
                    }
                }
            };
        });
        closest
    }
}