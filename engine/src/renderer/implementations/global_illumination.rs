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
    const EPSILON: f64 = 1e-6;

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

    fn _ambient(&self, ray: &Ray, hit: &Option<Hit>) -> RgbIntensity {
        self.ambient.evaluate(ray, hit)
    }

    fn _point_light_intensity(&self, light: &PointLight, hit: &Hit) -> RgbIntensity {
        let dir_unnormed = light.position - hit.pos;
        let distance = dir_unnormed.magnitude();
        let dir = Unit::new_normalize(dir_unnormed);

        let mut light_ray = Ray::new(hit.pos, dir, 1.);
        let mut light_absorbed: RgbIntensity = [1.; 3].into();

        while let Some(hit) = self.scene.intersect(&light_ray) {
            if hit.material.transmission {
                if hit.normal.dot(&dir) < 0. {
                    light_absorbed = light_absorbed
                        .component_mul(&hit.material.transmittance)
                        .component_mul(&hit.material.color);
                }
            } else {
                light_absorbed = RgbIntensity::zeros();
                break;
            }

            light_ray.origin = hit.pos + dir.scale(Self::EPSILON);
        }
        light.color.component_mul(&light_absorbed) * light.intensity
            / (distance as f32 + 1.).powf(2.)
    }

    fn _light_exposure(&self, ray: &Ray, hit: &Hit) -> RgbIntensity {
        self.light_list
            .iter()
            .map(|light| {
                let light_vector = (light.position - hit.pos).normalize();

                (hit.material.roughness * (hit.normal.dot(&light_vector).max(0.0) as f32)
                    + hit.material.metallic
                        * (ray
                            .reflected_dir(&hit.normal)
                            .dot(&light_vector)
                            .max(0.0)
                            .powf(hit.material.k) as f32))
                    .component_mul(&self._point_light_intensity(light, hit))
            })
            .sum()
    }

    fn _cast(&self, ray: &Ray, depth: usize, mut ior_stack: Vec<f64>) -> RgbIntensity {
        let ray_hit = self.scene.intersect(ray);
        let mut intensity = self._ambient(ray, &ray_hit);

        if let Some(hit) = ray_hit {
            intensity += self._light_exposure(ray, &hit);

            if depth < self.bounce_limit {
                let reflected_dir = ray.reflected_dir(&hit.normal);
                intensity += self
                    ._cast(
                        &Ray::new(hit.pos + reflected_dir.scale(Self::EPSILON), reflected_dir, ray.ior),
                        depth + 1,
                        ior_stack.clone(),
                    )
                    .component_mul(&hit.material.metallic);

                if hit.material.transmission {
                    let ior = if hit.normal.dot(&ray.direction) <= 0. { // todo rethink
                        ior_stack.push(hit.material.ior);
                        hit.material.ior
                    } else {
                        ior_stack.pop().unwrap()
                    };

                    if let Some(refracted_dir) = ray.refracted_dir(&hit.normal, ior) {
                        intensity += self
                            ._cast(&Ray::new(hit.pos + refracted_dir.scale(Self::EPSILON), refracted_dir, ior), depth + 1, ior_stack)
                            .component_mul(&hit.material.transmittance);
                    }
                }
            }
            intensity = intensity.component_mul(&hit.material.color);
        }
        intensity
    }
}

impl<M: Model, A: Ambient> Renderer for GlobalIllumination<M, A> {
    fn cast(&self, ray: &Ray) -> RgbIntensity {
        self._cast(ray, 0, [1.0].into())
    }
}
