#![allow(unused)]

use crate::renderer::Renderer;
use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::material::Rgb;
use crate::renderer::objects::model::Model;
use crate::renderer::objects::ray::{Ray, Unit, Unit3, Vector3, multiply_rgb, saturating_add};
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
    rng: R
}

impl<M: Model, E: Environment, R: Rng + Clone> Sampling<M, E, R> {
    pub fn new(scene: Scene<M>, environment: E, bounce_limit: usize, rng: R) -> Self {
        Self {
            scene,
            environment,
            bounce_limit,
            rng
        }
    }

    fn diffused_dir(norm: &Unit, rng: &mut R) -> Unit {
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

        let r1 = rng.random::<f64>();
        let r2 = rng.random::<f64>() * std::f64::consts::TAU;

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

    fn define_new_ray<'a>(original: &'a mut Ray, hit: &Hit, rng: &mut R) -> &'a Ray {
        let specular = rng.random::<u8>();

        original.direction = if specular <= hit.material.metallic.x {
            Self::reflected_ray(&original.direction, &hit.normal)
        } else {
            Self::diffused_dir(&hit.normal, rng)
        };
        original.origin = hit.pos;
        original
    }
}

impl<M: Model, E: Environment, R: Rng + Clone> Renderer for Sampling<M, E, R> {
    fn cast(&self, ray: &Ray) -> Rgb {
        let mut rng = self.rng.clone();

        let mut current_ray = ray.clone();

        let mut color = Rgb::new(0xFF, 0xFF, 0xFF);
        let mut emission_collected = Rgb::new(0, 0, 0);

        for _ in 0..self.bounce_limit {
            if let Some(hit) = self.scene.intersect(&current_ray) {
                Self::define_new_ray(&mut current_ray, &hit, &mut rng);
                color = multiply_rgb(&hit.material.color, &color);
                emission_collected = saturating_add(&emission_collected, &hit.material.emissivity);
            } else {
                color = multiply_rgb(&self.environment.evaluate(&current_ray), &color);
                break;
            }
        }

        saturating_add(&color, &emission_collected)
    }
}

#[cfg(test)]
mod tests {

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::renderer::objects::model::sphere::SphereModel;
    use crate::renderer::objects::ray::Vector;
    use super::*;

    #[test]
    fn test_diffuse() {
        let mut rng = ChaCha8Rng::seed_from_u64(0);

        let norm_x = Vector::x_axis();
        let norm_y = Vector::y_axis();
        let norm_z = Vector::z_axis();
        let r_norm = Unit::new_normalize([1., 1., -1., 0.].into());
        for _ in 0..100 {
            let x_diffuse = Sampling::<SphereModel, Black, ChaCha8Rng>::diffused_dir(&norm_x, &mut rng);
            assert!(x_diffuse.x > 0.);
            let y_diffuse = Sampling::<SphereModel, Black, ChaCha8Rng>::diffused_dir(&norm_y, &mut rng);
            assert!(y_diffuse.y > 0.);
            let z_diffuse = Sampling::<SphereModel, Black, ChaCha8Rng>::diffused_dir(&norm_z, &mut rng);
            assert!(z_diffuse.z > 0.);
            let r_diffuse = Sampling::<SphereModel, Black, ChaCha8Rng>::diffused_dir(&r_norm, &mut rng);
            assert!(r_diffuse.dot(&r_norm) >= 0.);
        }
    }
}
