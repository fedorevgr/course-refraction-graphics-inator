#![allow(dead_code)]

use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::material::RgbIntensity;
use crate::renderer::objects::model::Model;
use crate::renderer::objects::ray::{Ray, Rgb, Vector};
use crate::renderer::Renderer;
use crate::renderer::scene::Scene;

struct PointLight {
    color: RgbIntensity,
    intensity: f32,
}

pub trait Ambient {
    fn evaluate(&self, ray: &Ray) -> RgbIntensity;
}

pub struct WithSky {}
impl Ambient for WithSky {
    fn evaluate(&self, ray: &Ray) -> RgbIntensity {
        if ray.direction.z < 0.0 {
            RgbIntensity::zeros()
        } else {
            RgbIntensity::new(0.0, 0.0, 1.0)
        }
    }
}

pub struct GlobalIllumination<M: Model, A: Ambient> {
    light_list: Vec<PointLight>, 
    scene: Scene<M>,
    bounce_limit: usize,
    ambient: A,
}

impl<M: Model, A: Ambient> GlobalIllumination<M, A> {
    pub fn new(scene: Scene<M>, light_list: Vec<PointLight>, bounce_limit: usize, ambient: A) -> Self {
        Self { scene, light_list, bounce_limit, ambient }
    }

    fn _reflected(ray: &Ray, hit: &Hit ) -> Ray {
        todo!()
    }

    fn _refracted(ray: &Ray, hit: &Hit) -> Option<Ray> {
        todo!()
    }

    fn _specular(&self, ray: &Ray, hit: &Hit) -> RgbIntensity {
        todo!()
    }

    fn _diffusive(&self, ray: &Ray, hit: &Hit) -> RgbIntensity {
        todo!()
    }

    fn _ambient(&self, ray: &Ray, hit: &Hit) -> RgbIntensity {
        self.ambient.evaluate(&ray)
    }

    fn _cast(&self, ray: &Ray, depth: usize) -> RgbIntensity {

        let hit = self.scene.intersect(ray);

        if let Some(hit) = hit {
            let mut intensity = self._ambient(ray, &hit) + self._diffusive(ray, &hit) + self._specular(ray, &hit);

            if depth < self.bounce_limit {
                intensity += self._cast(&Self::_reflected(ray, &hit), depth + 1);

                if let Some(refracted) = Self::_refracted(ray, &hit) {
                    intensity += self._cast(&refracted, depth + 1);
                }
            }
            intensity
        }
        else {
            RgbIntensity::zeros()
        }

    }
}

impl<M: Model, A: Ambient> Renderer for GlobalIllumination<M, A> {
    fn cast(&self, ray: &Ray) -> RgbIntensity {
        self._cast(ray, 0)
    }
}
