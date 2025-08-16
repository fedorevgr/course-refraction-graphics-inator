#![allow(dead_code)]

pub use crate::renderer::objects::ray::{RgbIntensity};
use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
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
    pub transmittance: RgbIntensity,

    #[builder(default = RgbIntensity::from([0.; 3]))]
    pub ambient: RgbIntensity,

    #[builder(default = 0.0)]
    pub k: f64,

    #[builder(default = 1.0)]
    pub env: f64
}

impl Material {
    pub fn metallic() -> Self {
        Material {
            color: RgbIntensity::from([0.5, 0.5, 0.7]),
            emissivity: RgbIntensity::from([0.; 3]),
            metallic: RgbIntensity::from([1.; 3]),
            roughness: RgbIntensity::from([0.2; 3]),
            transmittance: RgbIntensity::from([0.; 3]),
            ambient: RgbIntensity::from([0.; 3]),
            k: 30.,
            env: 1.,
        }
    }

    pub fn marble() -> Self {
        Material {
            color: RgbIntensity::from([0.8; 3]),
            emissivity: RgbIntensity::from([0.; 3]),
            metallic: RgbIntensity::from([0.; 3]),
            roughness: RgbIntensity::from([0.7; 3]),
            transmittance: RgbIntensity::from([0.; 3]),
            ambient: RgbIntensity::from([0.; 3]),
            k: 2.,
            env: 1.,
        }
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
            env: 1.,
        }
    }
}
