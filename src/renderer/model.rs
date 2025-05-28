use vector3d::Vector3d;
use crate::renderer::ray::Vector;
use super::ray;


struct Triangle {
    normal: Vector,
    idx: [usize; 3],
}

struct Model {
    points: Vec<Vector>,
    triangles: Vec<Triangle>,
}

impl Model {
    fn contruct(points: Vec<Vector>, triangles: Vec<Triangle>) -> Model {
        Model { points, triangles }
    }
    
}