use vector3d::Vector3d;

pub type Vector = Vector3d<f64>;

struct Ray {
    origin: Vector,
    direction: Vector
}

impl Ray {
    fn construct(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }
}