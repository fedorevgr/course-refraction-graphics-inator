#![allow(dead_code)]

pub use crate::renderer::objects::ray::{Rgb};
use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
pub struct Material {
    #[builder(default = Rgb::from([0xC0; 3]))]
    pub color: Rgb,

    #[builder(default = Rgb::from([0x0; 3]))]
    pub emissivity: Rgb,

    #[builder(default = Rgb::from([0x10; 3]))]
    pub metallic: Rgb,

    #[builder(default = Rgb::from([0xC0; 3]))]
    pub roughness: Rgb,

    #[builder(default = Rgb::from([0x0; 3]))]
    pub transmittance: Rgb,

    #[builder(default = 0.0)]
    pub k: f64
}

impl Material {
    pub fn metallic() -> Self {
        Material {
            color: Rgb::from([0x80, 0x80, 0xC0]),
            emissivity: Rgb::from([0; 3]),
            metallic: Rgb::from([0xFF; 3]),
            roughness: Rgb::from([0x30; 3]),
            transmittance: Rgb::from([0; 3]),
            k: 30.
        }
    }

    pub fn marble() -> Self {
        Material {
            color: Rgb::from([0xA0; 3]),
            emissivity: Rgb::from([0; 3]),
            metallic: Rgb::from([0; 3]),
            roughness: Rgb::from([0xCF; 3]),
            transmittance: Rgb::from([0; 3]),
            k: 2.
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material{
            color: Rgb::from([0; 3]),
            emissivity: Rgb::from([0; 3]),
            metallic: Rgb::from([0xFF; 3]),
            roughness: Rgb::from([0xFF; 3]),
            transmittance: Rgb::from([0; 3]),
            k: 0.
        }
    }
}
