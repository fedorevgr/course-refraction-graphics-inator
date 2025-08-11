#![allow(unused)]

use std::cell::RefCell;
use crate::renderer::Renderer;
use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::material::Rgb;
use crate::renderer::objects::model::Model;
use crate::renderer::objects::ray::{Ray, Unit, Unit3, Vector3, multiply_rgb, saturating_add, Vector};
use crate::renderer::scene::Scene;
use rand::prelude::Rng;

pub trait Environment {
    fn evaluate(&self, ray: &Ray) -> Rgb;
}

#[derive(Clone)]
pub struct Black {}

impl Environment for Black {
    fn evaluate(&self, ray: &Ray) -> Rgb {
        Rgb::zeros()
    }
}

#[derive(Debug, Clone)]
pub struct Sampling<M: Model, E: Environment, R: Rng + Clone> {
    scene: Scene<M>,
    environment: E,
    bounce_limit: usize,
    rng: RefCell<R>
}

impl<M: Model, E: Environment, R: Rng + Clone> Sampling<M, E, R> {
    pub fn new(scene: Scene<M>, environment: E, bounce_limit: usize, rng: R) -> Self {
        Self {
            scene,
            environment,
            bounce_limit,
            rng: RefCell::new(rng)
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
}

impl<M: Model, E: Environment, R: Rng + Clone> Renderer for Sampling<M, E, R> {
    fn cast(&self, ray: &Ray) -> Rgb {
        let mut dbg = false;
        if dbg {
            println!("New ray");
        }

        let mut current_ray = ray.clone();

        let mut color = Rgb::new(0xFF, 0xFF, 0xFF);
        let mut emission_collected = Rgb::new(0, 0, 0);

        for _ in 0..self.bounce_limit {
            if dbg {
                println!("{:.3?}", current_ray);
            }
            if let Some(hit) = self.scene.intersect(&current_ray) {
                current_ray = self.define_new_ray(&mut current_ray, &hit);

                color = multiply_rgb(&hit.material.color, &color);
                emission_collected = saturating_add(&emission_collected, &hit.material.emissivity);
            } else {
                emission_collected = saturating_add(&self.environment.evaluate(&current_ray), &emission_collected);
                break;
            }
        }
        
        multiply_rgb(&emission_collected, &color)
    }
}

#[cfg(test)]
mod tests {


}
