pub mod scene;
pub mod objects;

use scene::Scene;

use objects::ray::{Ray, Rgb, S, W};
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
    light: Vector
}

impl<M: Model> SimpleRenderer<M> {    
    pub fn new(scene: Scene<M>) -> SimpleRenderer<M> {
        SimpleRenderer {
            scene,
            background: Material::default(),
            light: Vector::new(10., 10., 10., 0.)
        }
    }
}

impl<M: Model> Renderer for SimpleRenderer<M> {
    fn cast(&self, ray: &Ray) -> Rgb {
        match self.scene.intersect(ray) {
            None => { self.background.color },
            Some(hit) => {
                let cos_reflection = W(((self.light - hit.pos).normalize().dot(&hit.normal).max(0.) * 256.) as u8);
                let cos_diffusive = W((ray.direction.dot(&-hit.normal).max(0.) * 256.) as u8);

                hit.material.color.component_mul(&(hit.material.roughness * cos_diffusive)) +
                     Rgb::new(S(255), S(255), S(255)).component_mul(&(hit.material.metallic * cos_reflection))
            }
        }
    }
}
