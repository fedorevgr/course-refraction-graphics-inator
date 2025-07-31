use crate::renderer::objects::model::{Model, SphereModel};


#[derive(Clone, Debug)]
pub struct Scene<M: Model> {
    pub objects: Vec<M>
}

impl<M: Model> Scene<M> {
    pub fn new(objects: Vec<M>) -> Self {
        todo!()
    }
}