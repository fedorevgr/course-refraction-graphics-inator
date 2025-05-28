use vector3d::Vector3d;
use crate::renderer::ray::Vector;
use super::ray;


#[derive(Debug)]
pub struct Triangle {
    pub normal: Vector,
    pub idx: [usize; 3],
}

#[derive(Debug)]
pub struct Model {
    points: Vec<Vector>,
    triangles: Vec<Triangle>,
}

impl Model {
    pub fn new(points: Vec<Vector>, triangles: Vec<Triangle>) -> Model {
        Model { points, triangles }
    }
    
}