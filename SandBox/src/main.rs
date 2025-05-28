use std;
use std::ops::Mul;
use nalgebra as na;

fn main() {
    let vector = na::Vector4::new(0., 1., 0., 0.);
    let matrix = na::Matrix4::new_rotation(na::Vector3::new(std::f64::consts::PI / 4.0, 0., 0.));
    println!("{:?}", matrix);
    println!("{:#?}", matrix * vector);

    println!("{:#?}", na::Matrix3::new_rotation(1.57));
}
