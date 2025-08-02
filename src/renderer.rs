pub mod scene;
pub mod objects;
pub mod color;

use scene::Scene;

use objects::ray::{Ray, Rgb};
use objects::model::Model;
use objects::material::Material;
use objects::ray::Vector;


pub trait Renderer {
    fn cast(&self, ray: &Ray) -> Rgb;
}

#[derive(Clone, Debug)]
pub struct SimpleRenderer<M: Model> {
    scene: Scene<M>,
    background: Material,
    light: Vector,
    light_color: Rgb
}

impl<M: Model> SimpleRenderer<M> {    
    pub fn new(scene: Scene<M>) -> SimpleRenderer<M> {
        SimpleRenderer {
            scene,
            background: Material::default(),
            light: Vector::new(10., -10., 10., 0.),
            light_color: Rgb::new(255, 255, 255)
        }
    }
}

// todo: move
fn multiply_high_byte(a: u8, b: u8) -> u8 {
    (((a as u16) * (b as u16) ) >> 8) as u8
}

impl<M: Model> Renderer for SimpleRenderer<M> {
    fn cast(&self, ray: &Ray) -> Rgb {
        match self.scene.intersect(ray) {
            None => { self.background.color },
            Some(hit) => {
                let cos_reflection = ((self.light - hit.pos).normalize().dot(&hit.normal).max(0.) * 255.) as u8;

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
