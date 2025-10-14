#![allow(dead_code)]

pub use crate::renderer::objects::ray::{RgbIntensity};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Material {
    #[builder(default = RgbIntensity::from([0.; 3]))]
    pub color: RgbIntensity,

    #[builder(default = RgbIntensity::from([0.; 3]))]
    pub emissivity: RgbIntensity,

    #[builder(default = RgbIntensity::from([0.; 3]))]
    pub metallic: RgbIntensity,

    #[builder(default = RgbIntensity::from([0.; 3]))]
    pub roughness: RgbIntensity,

    #[builder(default = RgbIntensity::from([0.; 3]))]
    pub ambient: RgbIntensity,

    #[builder(default = 1.)]
    pub k: f64,

    #[builder(default = 1.0)]
    pub ior: f64,
    
    #[builder(default = false)]
    pub transmission: bool,
    
    #[builder(default = RgbIntensity::from([0.; 3]))]
    pub transmittance: RgbIntensity,
}

impl Material {
    pub fn metallic() -> Self {
        MaterialBuilder::default()
            .color([0.5, 0.5, 0.7].into())
            .metallic([1.; 3].into())
            .k(10.)
            .build().unwrap()
    }

    pub fn marble() -> Self {
        MaterialBuilder::default()
            .color([0.8; 3].into())
            .roughness([0.7; 3].into())
            .ambient([0.2; 3].into())
            .metallic([0.1; 3].into())
            .build().unwrap()
    }
}

impl Default for Material {
    fn default() -> Self {
        Material{
            color: RgbIntensity::from([1.; 3]),
            emissivity: RgbIntensity::from([0.; 3]),
            metallic: RgbIntensity::from([0.3; 3]),
            roughness: RgbIntensity::from([0.2; 3]),
            transmittance: RgbIntensity::from([0.; 3]),
            ambient: RgbIntensity::from([0.; 3]),
            k: 0.,
            ior: 1.,
            transmission: false,
        }
    }
}
