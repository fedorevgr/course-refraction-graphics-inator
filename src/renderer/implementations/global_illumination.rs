#![allow(dead_code)]

use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::material::RgbIntensity;
use crate::renderer::objects::model::Model;
use crate::renderer::objects::ray::{Ray, Rgb, Vector};
use crate::renderer::Renderer;
use crate::renderer::scene::Scene;

pub struct PointLight {
    position: Vector,
    color: RgbIntensity,
    intensity: f32,
}

impl PointLight {
    pub fn new(position: Vector, intensity: f32, color: RgbIntensity) -> Self {
        PointLight {position, color, intensity}
    }
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
pub struct Solid {
    color: RgbIntensity,
}
impl Solid {
    pub fn new(color: RgbIntensity) -> Self {
        Solid {color}
    }
}
impl Ambient for Solid {
    fn evaluate(&self, ray: &Ray) -> RgbIntensity {
        self.color
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

    fn _specular(&self, ray: &Ray, hit: &Hit) -> RgbIntensity {
        let reflected = ray.reflected_dir(&hit.normal);

        self.light_list.iter().map(|light| {
            let light_vector = (light.position - ray.origin).normalize();
            light_vector.dot(&reflected).max(0.).abs().powf(hit.material.k) as f32 * light.intensity * light.color.component_mul(&hit.material.metallic)
        }).sum()
    }

    fn _diffusive(&self, ray: &Ray, hit: &Hit) -> RgbIntensity {
        self.light_list.iter().map(|light| {
            let light_vector = (light.position - ray.origin).normalize();
            light_vector.dot(&hit.normal).max(0.).abs() as f32 * light.intensity * light.color.component_mul(&hit.material.roughness)
        }).sum()
    }

    fn _ambient(&self, ray: &Ray, hit: &Hit) -> RgbIntensity {
        self.ambient.evaluate(&ray).component_mul(&hit.material.ambient)
    }

    fn _cast(&self, ray: &Ray, depth: usize) -> RgbIntensity {

        let hit = self.scene.intersect(ray);

        if let Some(hit) = hit {
            let mut intensity = self._ambient(ray, &hit) + self._diffusive(ray, &hit) + self._specular(ray, &hit);

            if depth < self.bounce_limit {
                intensity += self._cast(&Ray::new(hit.pos, ray.reflected_dir(&hit.normal), ray.env), depth + 1);

                if let Some(refracted_dir) = ray.refracted_dir(&hit.normal, hit.material.env) {
                    intensity += self._cast(&Ray::new(hit.pos, refracted_dir, hit.material.env), depth + 1);
                }
            }
            intensity
        }
        else {
            self.ambient.evaluate(&ray)
        }

    }
}

impl<M: Model, A: Ambient> Renderer for GlobalIllumination<M, A> {
    fn cast(&self, ray: &Ray) -> RgbIntensity {
        self._cast(ray, 0)
    }
}
