use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::model::{Model};
use crate::renderer::objects::ray::Ray;

#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl<M: Model + for<'de> Deserialize<'de>> Scene<M> {
    pub fn load_scene(data: &String) -> Result<Self, Box<dyn Error>> {
        serde_yaml::from_str::<Scene<M>>(data.as_str()).map_err(|e| e.into())
    }
}

impl<M: Model + Serialize> Scene<M> {
    pub fn save_scene(&self) -> Result<String, Box<dyn Error>> {
        serde_yaml::to_string(&self).map_err(|e| e.into())
    }
}
