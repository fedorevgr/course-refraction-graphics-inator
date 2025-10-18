#![allow(dead_code)]

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::renderer::implementations::global_illumination::PointLight;
use crate::renderer::objects::camera::Camera;
use crate::renderer::objects::camera::perspective::PerspectiveCamera;
use crate::renderer::objects::model::triangle::TriangleModel;
use crate::renderer::scene::Scene;

#[derive(Clone, Debug, Serialize,Deserialize, Builder)]
pub struct GlobalIlluminationCollection {
    pub lights: Vec<PointLight>,
    pub cameras: Vec<PerspectiveCamera>,
    pub scene: Scene<TriangleModel>
}

impl GlobalIlluminationCollection {
    pub fn load(data: &String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut collection: Self = serde_yaml::from_str(data)?;
        collection.scene.objects = collection.scene.objects.iter().map(|obj| { obj.clone().load_file().unwrap() }).collect();
        Ok(collection)
    }

    pub fn save(&self) -> Result<String, Box<dyn std::error::Error>> {
        serde_yaml::to_string(&self).map_err(|e| e.into())
    }
}