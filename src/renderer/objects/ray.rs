use std::convert::Into;
use nalgebra::{Matrix4, Vector4};
use nalgebra::Vector3 as V3;
use nalgebra::Unit as U;

pub type Vector = Vector4<f64>;
pub type Vector3 = V3<f64>;
pub type Unit = U<Vector>;
pub type Unit3 = U<Vector3>;

pub type Matrix = Matrix4<f64>;

pub type RgbIntensity = V3<f32>;

pub struct Rgb(pub RgbIntensity);
impl Rgb {

    #[inline]
    fn _convert(v: f32) -> u8 {
        (255. * v.clamp(0.0, 1.0)) as u8
    }

    #[inline]
    pub fn to_pixel(&self) -> [u8; 3]
    {
        [
            Self::_convert(self.0[0]),
            Self::_convert(self.0[1]),
            Self::_convert(self.0[2])
        ]
    }
    
    #[inline]
    pub fn from_pixel(v: [u8; 3]) -> RgbIntensity {
        RgbIntensity::new(v[0] as f32 / 255., v[1] as f32 / 255., v[2] as f32 / 255.)    
    }
}

// #[inline]
// pub fn multiply_high_byte(a: u8, b: u8) -> u8 {
//     (((a as u16) * (b as u16) ) >> 8) as u8
// }
// pub fn multiply_rgb(a: &RgbIntensity, b: &RgbIntensity) -> RgbIntensity {
//     RgbIntensity::new(
//         multiply_high_byte(a.x, b.x),
//         multiply_high_byte(a.y, b.y),
//         multiply_high_byte(a.z, b.z)
//     )
// }
//
// pub fn scale_rgb(a: &RgbIntensity, b: u8) -> RgbIntensity {
//     RgbIntensity::new(
//         multiply_high_byte(a.x, b),
//         multiply_high_byte(a.y, b),
//         multiply_high_byte(a.z, b)
//     )
// }
//
// pub fn saturating_add(a: &RgbIntensity, b: &RgbIntensity) -> RgbIntensity {
//     RgbIntensity::new(
//         a.x.saturating_add(b.x),
//         a.y.saturating_add(b.y),
//         a.z.saturating_add(b.z)
//     )
// }

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Unit,
    pub env: f64
}

impl Ray {

    pub fn new(origin: Vector, direction: Unit, env: f64) -> Ray {
        Ray { origin, direction, env }
    }

    pub fn refracted_dir(&self, normal: &Unit, env_nu: f64) -> Option<Unit>
    {
        let r = self.env / env_nu;
        let c = -self.direction.dot(&normal);
        let d = 1. - r * r * (1.0 - c * c);

        if d <= 0. {
            None
        }
        else {
            Some(Unit::new_unchecked(
                self.direction.scale(r) + normal.scale(r*c - d.sqrt())
            ))
        }
    }

    pub fn reflected_dir(&self, normal: &Unit) -> Unit
    {
        Unit::new_unchecked(
            self.direction.into_inner() + normal.scale(-2. * self.direction.dot(normal))
        )
    }

}

