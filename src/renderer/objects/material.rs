#[allow(dead_code)]

use crate::renderer::objects::ray::{Rgb, S, FLike, W};

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Rgb, // main color
    pub emissivity: FLike, // shine color

    pub metallic: FLike,
    pub roughness: FLike, // reflect note: RGB
    pub transmittance: FLike  // refract note: RGB
}

impl Material {
    pub fn metallic() -> Self {
        Material {
            color: Rgb::from([S(0x80), S(0x80), S(0xA0)]),
            emissivity: FLike::from([W(0); 3]),
            metallic: FLike::from([W(0xE0); 3]),
            roughness: FLike::from([W(0); 3]),
            transmittance: FLike::from([W(0); 3])
        }
    }

    pub fn marble() -> Self {
        Material {
            color: Rgb::from([S(0xA0); 3]),
            emissivity: FLike::from([W(0); 3]),
            metallic: FLike::from([W(0); 3]),
            roughness: FLike::from([W(0xCF); 3]),
            transmittance: FLike::from([W(0); 3])
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material{
            color: Rgb::from([S(0); 3]),
            emissivity: FLike::from([W(0); 3]),
            metallic: FLike::from([W(0xFF); 3]),
            roughness: FLike::from([W(0xFF); 3]),
            transmittance: FLike::from([W(0); 3])
        }
    }
}
