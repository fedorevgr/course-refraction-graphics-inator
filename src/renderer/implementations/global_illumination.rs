#![allow(dead_code)]

use crate::renderer::Renderer;
use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::material::RgbIntensity;
use crate::renderer::objects::model::Model;
use crate::renderer::objects::ray::{Ray, Rgb, Vector};
use crate::renderer::scene::Scene;
use nalgebra::Unit;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PointLight {
    position: Vector,
    color: RgbIntensity,
    intensity: f32,
}

impl PointLight {
    pub fn new(position: Vector, intensity: f32, color: RgbIntensity) -> Self {
        PointLight {
            position,
            color,
            intensity,
        }
    }
}

pub trait Ambient {
    fn evaluate(&self, ray: &Ray, hit: &Option<Hit>) -> RgbIntensity;
}

#[derive(Clone, Debug)]
pub struct WithSky {}
impl Ambient for WithSky {
    fn evaluate(&self, ray: &Ray, hit: &Option<Hit>) -> RgbIntensity {
        let free = if hit.is_some() { -1.0 } else { 1.0 };
        if ray.direction.z * free < 0.0 {
            [0.1; 3].into()
        } else {
            RgbIntensity::new(0.7, 0.8, 1.0)
        }
    }
}
#[derive(Clone)]
pub struct Solid {
    color: RgbIntensity,
}
impl Solid {
    pub fn new(color: RgbIntensity) -> Self {
        Solid { color }
    }
}
impl Ambient for Solid {
    fn evaluate(&self, _ray: &Ray, _hit: &Option<Hit>) -> RgbIntensity {
        self.color
    }
}

#[derive(Clone, Debug)]
pub struct GlobalIllumination<M: Model, A: Ambient> {
    light_list: Vec<PointLight>,
    scene: Scene<M>,
    bounce_limit: usize,
    ambient: A,
}

impl<M: Model, A: Ambient> GlobalIllumination<M, A> {
    pub fn new(
        scene: Scene<M>,
        light_list: Vec<PointLight>,
        bounce_limit: usize,
        ambient: A,
    ) -> Self {
        Self {
            scene,
            light_list,
            bounce_limit,
            ambient,
        }
    }

    fn _specular(&self, ray: &Ray, hit: &Hit) -> RgbIntensity {
        let reflected = ray.reflected_dir(&hit.normal);

        self.light_list
            .iter()
            .map(|light| {
                let light_vector = (light.position - hit.pos).normalize();
                let cosine = light_vector
                    .normalize()
                    .dot(&reflected)
                    .max(0.)
                    .abs()
                    .powf(hit.material.k) as f32;
                cosine
                    * light.intensity
                    * self
                        ._point_light_intensity(light, hit)
                        .component_mul(&hit.material.metallic)
            })
            .sum()
    }

    fn _diffusive(&self, _ray: &Ray, hit: &Hit) -> RgbIntensity {
        self.light_list
            .iter()
            .map(|light| {
                let light_vector = (light.position - hit.pos).normalize();
                let cosine = light_vector.dot(&hit.normal).max(0.).abs() as f32;
                cosine
                    * light.intensity
                    * self
                        ._point_light_intensity(light, hit)
                        .component_mul(&hit.material.roughness)
            })
            .sum()
    }

    fn _ambient(&self, ray: &Ray, hit: &Option<Hit>) -> RgbIntensity {
        self.ambient.evaluate(ray, hit)
    }

    #[inline]
    fn _point_light_intensity(&self, light: &PointLight, hit: &Hit) -> RgbIntensity {
        let dir = light.position - hit.pos;
        let distance = dir.magnitude();

        let mut light_ray = Ray::new(hit.pos, Unit::new_normalize(dir), 1.);
        let mut light_absorbed: RgbIntensity = [1.; 3].into();

        while let Some(hit) = self.scene.intersect(&light_ray) {
            if hit.normal.dot(&dir) < 0. {
                light_absorbed = light_absorbed.component_mul(&hit.material.transmittance).component_mul(&hit.material.color);
            }
            light_ray.origin = hit.pos;

        }
        light.color.component_mul(&light_absorbed) / (distance as f32 + 1.).powf(2.)
    }

    fn _cast(&self, ray: &Ray, depth: usize) -> RgbIntensity {
        let ray_hit = self.scene.intersect(ray);
        let mut intensity = self._ambient(ray, &ray_hit);

        if let Some(hit) = ray_hit {
            intensity += self._diffusive(ray, &hit) + self._specular(ray, &hit);

            if depth < self.bounce_limit {
                intensity += self
                    ._cast(
                        &Ray::new(hit.pos, ray.reflected_dir(&hit.normal), ray.ior),
                        depth + 1,
                    )
                    .component_mul(&hit.material.metallic);

                let ior = if hit.normal.dot(&ray.direction) <= 0. {
                    hit.material.ior
                } else {
                    1.0
                };
                if let Some(refracted_dir) = ray.refracted_dir(&hit.normal, ior) {
                    intensity += self
                        ._cast(&Ray::new(hit.pos, refracted_dir, ior), depth + 1)
                        .component_mul(&hit.material.transmittance);
                }
            }
            intensity = intensity.component_mul(&hit.material.color);
        }
        intensity
    }
}

impl<M: Model, A: Ambient> Renderer for GlobalIllumination<M, A> {
    fn cast(&self, ray: &Ray) -> RgbIntensity {
        self._cast(ray, 0)
    }
}
