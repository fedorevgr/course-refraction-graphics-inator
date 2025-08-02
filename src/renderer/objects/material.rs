#[allow(dead_code)]

pub use crate::renderer::objects::ray::{Rgb};
use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
pub struct Material {
    pub color: Rgb,
    pub emissivity: Rgb,

    pub metallic: Rgb,
    pub roughness: Rgb,
    pub transmittance: Rgb,

    pub k: f64
}

impl Material {
    pub fn metallic() -> Self {
        Material {
            color: Rgb::from([0x80; 3]),
            emissivity: Rgb::from([0; 3]),
            metallic: Rgb::from([0xE0; 3]),
            roughness: Rgb::from([0; 3]),
            transmittance: Rgb::from([0; 3]),
            k: 4.
        }
    }

    pub fn marble() -> Self {
        Material {
            color: Rgb::from([0xA0; 3]),
            emissivity: Rgb::from([0; 3]),
            metallic: Rgb::from([0; 3]),
            roughness: Rgb::from([0xCF; 3]),
            transmittance: Rgb::from([0; 3]),
            k: 0.
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
