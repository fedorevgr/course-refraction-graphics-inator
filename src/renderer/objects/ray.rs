use nalgebra::{Matrix4, Vector4};
use nalgebra::Vector3 as V3;
use nalgebra::Unit as U;

pub type Vector = Vector4<f64>;
pub type Vector3 = V3<f64>;
pub type Unit = U<Vector>;
pub type Unit3 = U<Vector3>;

pub type Matrix = Matrix4<f64>;

pub type Rgb = V3<u8>;

#[inline]
pub fn multiply_high_byte(a: u8, b: u8) -> u8 {
    (((a as u16) * (b as u16) ) >> 8) as u8
}

pub fn multiply_rgb(a: &Rgb, b: &Rgb) -> Rgb {
    Rgb::new(
        multiply_high_byte(a.x, b.x),
        multiply_high_byte(a.y, b.y),
        multiply_high_byte(a.z, b.z)
    )
}
pub fn saturating_add(a: &Rgb, b: &Rgb) -> Rgb {
    Rgb::new(
        a.x.saturating_add(b.x),
        a.y.saturating_add(b.y),
        a.z.saturating_add(b.z)
    )
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Unit
}

impl Ray {
    pub fn new(origin: Vector, direction: Unit) -> Ray {
        Ray { origin, direction }
    }
}

