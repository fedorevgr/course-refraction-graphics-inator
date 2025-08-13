#![allow(unused)]

use std::cell::RefCell;
use nalgebra::Vector4;
use crate::renderer::Renderer;
use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::material::Rgb;
use crate::renderer::objects::model::Model;
use crate::renderer::objects::ray::{Ray, Unit, Unit3, Vector3, multiply_rgb, saturating_add, Vector, scale_rgb};
use crate::renderer::scene::Scene;
use rand::prelude::Rng;

pub trait Environment {
    fn evaluate(&self, ray: &Ray) -> Rgb;
}

#[derive(Clone)]
pub struct Black {}

impl Environment for Black {
    fn evaluate(&self, ray: &Ray) -> Rgb {
        [0x30; 3].into()
    }
}

#[derive(Debug, Clone)]
pub struct Sampling<M: Model, E: Environment, R: Rng + Clone> {
    scene: Scene<M>,
    environment: E,
    bounce_limit: usize,
    rng: RefCell<R>,
    samples: usize
}

impl<M: Model, E: Environment, R: Rng + Clone> Sampling<M, E, R> {
    pub fn new(scene: Scene<M>, environment: E, bounce_limit: usize, rng: R) -> Self {
        Self {
            scene,
            environment,
            bounce_limit,
            rng: RefCell::new(rng),
            samples: 50
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

        let r1 = self.rng.borrow_mut().random::<f64>();
        let r2 = self.rng.borrow_mut().random::<f64>() * std::f64::consts::TAU;

        let cos = r1.sqrt();
        let sin = (1. - r1).sqrt();

        Unit::new_unchecked(
            (norm3.scale(cos) + t1.scale(sin * r2.sin()) + t2.scale(sin * r2.cos()))
                .to_homogeneous(),
        )
    }

    fn reflected_ray(original: &Unit, norm: &Unit) -> Unit {
        Unit::new_unchecked(original.into_inner() + norm.scale(norm.dot(original) * -2.))
    }

    fn define_new_ray(&self, original: &Ray, hit: &Hit) -> Ray {
        let specular = self.rng.borrow_mut().random::<u8>();

        Ray {
            direction: if specular <= hit.material.metallic.x {
                Self::reflected_ray(&original.direction, &hit.normal)
            } else {
                self.diffused_dir(&hit.normal)
            },
            origin: hit.pos
        }
    }
    fn cast_once(&self, ray: &Ray) -> Rgb {
        let mut current_ray = ray.clone();

        let mut color = Rgb::new(0xFF, 0xFF, 0xFF);
        let mut emission_collected = Rgb::new(0, 0, 0);

        for _ in 0..self.bounce_limit {
            if let Some(hit) = self.scene.intersect(&current_ray) {
                current_ray = self.define_new_ray(&mut current_ray, &hit);

                let emitted = &hit.material.emissivity;
                emission_collected = saturating_add(&emission_collected, &multiply_rgb(&emitted, &color));
                color = multiply_rgb(&hit.material.color, &color);
            } else {
                emission_collected = saturating_add(&multiply_rgb(&self.environment.evaluate(&current_ray), &color), &emission_collected);
                break;
            }
        }

        emission_collected
    }
}

impl<M: Model, E: Environment, R: Rng + Clone> Renderer for Sampling<M, E, R> {
    fn cast(&self, ray: &Ray) -> Rgb {

        let mut rgb = nalgebra::Vector3::<usize>::zeros();
        for _ in 0..self.samples {
            rgb += self.cast_once(ray).map(|v| v as usize);
        }
        Rgb::new((rgb.x / self.samples) as u8, (rgb.y / self.samples) as u8, (rgb.z / self.samples) as u8)
    }
}

#[cfg(test)]
mod tests {


}
