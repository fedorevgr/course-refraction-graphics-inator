use crate::renderer::objects::material::Rgb;
use crate::renderer::objects::model::Model;
use crate::renderer::objects::ray::{Ray, Unit};
use crate::renderer::Renderer;
use crate::renderer::scene::Scene;

pub trait Environment {
    fn evaluate(&self, ray: &Ray) -> Rgb;
}

#[derive(Debug, Clone)]
pub struct DiffuseRenderer<M: Model, E: Environment>  {
    scene: Scene<M>,
    environment: E
}

impl<M: Model, E: Environment> DiffuseRenderer<M, E> {
    pub fn new(scene: Scene<M>, environment: E) -> Self {
        Self {
            scene,
            environment
        }
    }
    
    fn diffused_dir(original: &Unit, norm: &Unit) -> Unit {
        todo!()
    }

    fn reflected_ray(original: &Unit, norm: &Unit) -> Unit {
        Unit::new_unchecked(original.into_inner() + norm.scale(norm.dot(original) * 2.))
    }
}

impl<M: Model, E: Environment> Renderer for DiffuseRenderer<M, E> {
    fn cast(&self, ray: &Ray) -> Rgb {
        todo!()
    }
}