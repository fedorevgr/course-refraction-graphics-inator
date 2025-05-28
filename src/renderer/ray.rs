use vector3d::Vector3d;

pub type Vector = Vector3d<f64>;

pub struct Ray {
    origin: Vector,
    direction: Vector
}

impl Ray {
    fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }
}