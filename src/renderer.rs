pub mod scene;
pub mod objects;

use objects::ray::Ray;

use scene::Scene;
use crate::renderer::objects::model::Model;

pub trait Renderer {
    fn cast(ray: &Ray) -> image::Rgb<u8>;
}

#[derive(Clone, Debug)]
pub struct RayTracer<M: Model> {
    scene: Scene<M>
}

impl<M: Model> RayTracer<M> {
    pub fn new(scene: Scene<M>) -> RayTracer<M> {
        RayTracer { scene }
    }
}

impl<M: Model> Renderer for RayTracer<M> {
    fn cast(ray: &Ray) -> image::Rgb<u8> {
        todo!()
    }
}
