#![allow(unused)]

use std::cell::RefCell;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use nalgebra::Vector4;
use crate::renderer::Renderer;
use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::material::RgbIntensity;
use crate::renderer::objects::model::Model;
use crate::renderer::objects::ray::{Ray, Unit, Unit3, Vector3, Vector};
use crate::renderer::scene::Scene;
use rand::prelude::Rng;

pub trait Environment {
    fn evaluate(&self, ray: &Ray) -> RgbIntensity;
}

#[derive(Clone)]
pub struct Black {}

impl Environment for Black {
    fn evaluate(&self, ray: &Ray) -> RgbIntensity {
        [0.2; 3].into()
    }
}

#[derive(Debug, Clone)]
pub struct Sampling<M: Model, E: Environment, R: Rng> {
    scene: Scene<M>,
    environment: E,
    bounce_limit: usize,
    rng: Arc<Mutex<R>>,
    samples: usize
}

impl<M: Model, E: Environment, R: Rng> Sampling<M, E, R> {
    pub fn new(scene: Scene<M>, environment: E, bounce_limit: usize, rng: R, samples: usize) -> Self {
        Self {
            scene,
            environment,
            bounce_limit,
            rng: Arc::new(Mutex::new(rng)),
            samples
        }
    }

    fn diffused_dir(&self, norm: &Unit) -> Unit {
        let norm3 = Vector3::from_homogeneous(norm.into_inner()).unwrap();
        let t1 = Unit3::new_normalize(
            if (norm.z - 1.).abs() < 0.0001 {
                Vector3::y_axis()
            } else {
                Vector3::z_axis()
            }
            .cross(&norm3),
        );

        let t2 = Unit3::new_unchecked(t1.cross(&norm3));

        let mut rng = self.rng.lock().unwrap();
        let r1 = (*rng).random::<f64>();
        let r2 = (*rng).random::<f64>() * std::f64::consts::TAU;
        drop(rng);

        let cos = r1.sqrt();
        let sin = (1f64 - r1).sqrt();

        Unit::new_unchecked(
            (norm3.scale(cos) + t1.scale(sin * r2.sin()) + t2.scale(sin * r2.cos()))
                .to_homogeneous(),
        )
    }

    fn reflected_ray(original: &Unit, norm: &Unit) -> Unit {
        Unit::new_unchecked(original.into_inner() + norm.scale(norm.dot(original) * -2.))
    }

    fn define_new_ray(&self, original: &Ray, hit: &Hit) -> Ray {
        let specular = self.rng.lock().unwrap().random::<f32>();

        Ray {
            direction: if specular <= hit.material.metallic.x {
                Self::reflected_ray(&original.direction, &hit.normal)
            } else {
                self.diffused_dir(&hit.normal)
            },
            origin: hit.pos,
            ior: 1.
        }
    }
    fn cast_once(&self, ray: &Ray) -> RgbIntensity {
        let mut current_ray = ray.clone();

        let mut color = RgbIntensity::from([1.; 3]);
        let mut emission_collected = RgbIntensity::from([0.; 3]);

        for _ in 0..self.bounce_limit {
            if let Some(hit) = self.scene.intersect(&current_ray) {
                current_ray = self.define_new_ray(&current_ray, &hit);

                let emitted = &hit.material.emissivity;
                emission_collected += emitted.component_mul(&color);
                color = hit.material.color.component_mul(&color);
            } else {
                emission_collected = self.environment.evaluate(&current_ray).component_mul(&color) + emission_collected;
                break;
            }
        }

        emission_collected
    }
}

impl<M: Model, E: Environment, R: Rng> Renderer for Sampling<M, E, R> {
    fn cast(&self, ray: &Ray) -> RgbIntensity {
        (1. / self.samples as f32) * (0..self.samples).map(|_| self.cast_once(ray)).sum::<RgbIntensity>()
    }
}

#[cfg(test)]
mod tests {


}
