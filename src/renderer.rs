pub mod scene;
pub mod objects;

use image::flat::NormalForm::Unaliased;
use nalgebra::Unit;
use scene::Scene;

use objects::ray::Ray;
use objects::model::Model;
use crate::renderer::objects::hit::Hit;
use crate::renderer::objects::material::Material;
use crate::renderer::objects::ray::Vector;

pub trait Renderer {
    fn cast(&self, ray: &Ray) -> image::Rgb<u8>;
}

#[derive(Clone, Debug)]
pub struct SimpleRenderer<M: Model> {
    scene: Scene<M>,
    background: Material
}

impl<M: Model> SimpleRenderer<M> {    
    pub fn new(scene: Scene<M>) -> SimpleRenderer<M> {
        SimpleRenderer { scene, background: Material::default() }
    }
}

impl<M: Model> Renderer for SimpleRenderer<M> {
    fn cast(&self, ray: &Ray) -> image::Rgb<u8> {
        self.scene.intersect(ray).unwrap_or(
            Hit::new(
                0., 
                Vector::zeros(), 
                &self.background, 
                Unit::new_normalize(Vector::new(1., 0., 0., 0.))
            )
        ).material.color
    }
}
