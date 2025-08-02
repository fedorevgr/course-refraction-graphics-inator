use crate::renderer::objects::ray::RGB;

#[derive(Debug, Clone)]
pub struct Material {
    pub color: RGB, // main color
    pub emissivity: RGB, // shine color

    pub roughness: RGB, // reflect note: RGB
    pub transmittance: RGB  // refract note: RGB
}

impl Material {
    // todo
}

impl Default for Material {
    fn default() -> Self {
        Material{
            color: RGB::from([128, 128, 128]),
            emissivity: RGB::from([0, 0, 0]),
            roughness: RGB::from([255, 255, 255]),
            transmittance: RGB::from([0, 0, 0])
        }
    }
}
