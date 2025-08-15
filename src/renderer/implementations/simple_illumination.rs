#![allow(dead_code)]

use crate::renderer::objects::material::{Material, Rgb};
use crate::renderer::objects::model::Model;
use crate::renderer::objects::ray::{Ray, Vector, multiply_high_byte};
use crate::renderer::{Renderer};
use crate::renderer::scene::Scene;


#[derive(Clone, Debug)]
pub struct SimpleIllumination<M: Model> {
    scene: Scene<M>,
    background: Material,
    light: Vector,
    light_color: Rgb
}

impl<M: Model> SimpleIllumination<M> {
    pub fn new(scene: Scene<M>) -> SimpleIllumination<M> {
        SimpleIllumination {
            scene,
            background: Material::default(),
            light: Vector::new(10., -10., 10., 0.),
            light_color: Rgb::new(255, 255, 255)
        }
    }
}

impl<M: Model> Renderer for SimpleIllumination<M> {
    fn cast(&self, ray: &Ray) -> Rgb {
        match self.scene.intersect(ray) {
            None => { self.background.color },
            Some(hit) => {
                let cos_reflection = ((self.light - hit.pos).normalize().dot(&hit.normal).max(0.).powf(hit.material.k) * 255.) as u8;

                let cos_diffusive = (ray.direction.dot(&-hit.normal).max(0.) * 255.) as u8;

                let mut color_res = Rgb::zeros();

                for i in 0..3 {

                    let reflection_intensity = multiply_high_byte(multiply_high_byte(self.light_color[i], hit.material.metallic[i]), cos_reflection);
                    let diffusion_intensity = multiply_high_byte(multiply_high_byte(hit.material.color[i], hit.material.roughness[i]), cos_diffusive);
                    color_res[i] = diffusion_intensity.saturating_add(reflection_intensity);
                };

                color_res
            }
        }
    }
}