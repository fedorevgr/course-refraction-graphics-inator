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
    #[allow(dead_code)]
    pub fn from_pixel(v: [u8; 3]) -> RgbIntensity {
        RgbIntensity::new(v[0] as f32 / 255., v[1] as f32 / 255., v[2] as f32 / 255.)    
    }
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Unit,
    pub ior: f64
}

impl Ray {

    pub fn new(origin: Vector, direction: Unit, env: f64) -> Ray {
        Ray { origin, direction, ior: env }
    }

    pub fn refracted_dir(&self, normal: &Unit, env_nu: f64) -> Option<Unit>
    {
        let dot_prod = self.direction.dot(normal);
        let norm = if dot_prod >= 0. {normal.scale(-1.)} else { *normal.clone() };

        let r = self.ior / env_nu;
        let c = -self.direction.dot(&norm);

        let d = 1. - r * r * (1.0 - c * c);

        if d < 0. {
            None
        }
        else {
            Some(Unit::new_unchecked(
                self.direction.scale(r) + norm.scale(r * c - d.sqrt())
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


#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use crate::renderer::objects::ray::{Ray, Vector, Unit};

    #[test]
    fn test_refracted_direction() {
        let norm = Unit::new_normalize([0., 0., 1., 0.].into());
        let tangent = Unit::new_normalize([1., 1., 0., 0.].into());

        let start_ior = 1.0;
        let ior = 1.33;
        let start_angle = 30f64.to_radians();
        let end_angle = (start_ior / ior * start_angle.sin()).asin();

        let start_dir = -Unit::new_normalize(tangent.scale(start_angle.sin()) + norm.scale(start_angle.cos()));
        let start_ray = Ray::new([0.; 4].into(), start_dir, start_ior);
        let refracted = start_ray.refracted_dir(&norm, ior).unwrap();

        assert_relative_eq!(refracted, -Unit::new_normalize(tangent.scale(end_angle.sin()) + norm.scale(end_angle.cos())));
    }

    #[test]
    fn test_inner_refracted_direction() {
        let norm = Unit::new_normalize([0., 0., 1., 0.].into());
        let tangent = Unit::new_normalize([1., 1., 0., 0.].into());

        let start_ior = 1.33;
        let ior = 1.0;
        let start_angle = 30f64.to_radians();
        let end_angle = (start_ior / ior * start_angle.sin()).asin();

        let start_dir = -Unit::new_normalize(tangent.scale(start_angle.sin()) + norm.scale(start_angle.cos()));
        let start_ray = Ray::new([0.; 4].into(), start_dir, start_ior);
        let refracted = start_ray.refracted_dir(&norm, ior).unwrap();
        let refracted_2 = start_ray.refracted_dir(&-norm, ior).unwrap();

        assert_relative_eq!(refracted, -Unit::new_normalize(tangent.scale(end_angle.sin()) + norm.scale(end_angle.cos())));
        assert_relative_eq!(refracted, refracted_2);
    }

    #[test]
    fn test_direction_conservation() {
        let plane_normal = Unit::new_normalize([1., 1., 1., 0.].into());

        let direction = Unit::new_normalize([0., 0.,  -1., 0.].into());
        let direction_2 = Ray::new([0., 0., 0., 0.].into(), direction, 1.).refracted_dir(&plane_normal, 1.5).unwrap();
        let direction_res = Ray::new([0.; 4].into(), direction_2, 1.5).refracted_dir(&-plane_normal, 1.).unwrap();

        assert_relative_eq!(direction, direction_res);
    }
}
