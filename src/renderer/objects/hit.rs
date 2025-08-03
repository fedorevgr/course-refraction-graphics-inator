use nalgebra::Unit;
use crate::renderer::objects::material::Material;
use crate::renderer::objects::ray::Vector;

#[derive(Debug, Clone)]
pub struct Hit<'a> {
    pub factor: f64,
    pub pos: Vector,
    pub material: &'a Material,
    pub normal: Unit<Vector>,
}

impl<'a> Hit<'a> {
    pub fn new(
        factor: f64,
        pos: Vector,
        material: &'a Material,
        normal: Unit<Vector>,
    ) -> Self {
        Hit { factor, pos, material, normal }
    }
}
