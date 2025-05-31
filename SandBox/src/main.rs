use std::f64;
use nalgebra::{Vector3, Vector4};
use nalgebra::Matrix4;


const FOV_X: f64 = f64::consts::FRAC_PI_2;
const W: usize = 3;
const H: usize = 3;

const FOV_Y: f64 = FOV_X / W as f64 * H as f64;

const LIM_X: f64 = -FOV_X / 2.0;
const LIM_Y: f64 = -FOV_Y / 2.0;

const VECTOR: Vector4<f64> = Vector4::new(1., 0., 0., 0.);
const ANGLES: Vector3<f64> = Vector3::new(0., f64::consts::FRAC_PI_2, 0.);


fn proj(i: usize, j: usize) -> Option<Vector4<f64>> {
    if (i > W) || (j > H) {
        return None;
    }
    
    let delta_x = FOV_X / (W-1) as f64;
    let delta_y = FOV_Y / (H-1) as f64;
    
    let matrix_yaw = Matrix4::from_euler_angles(0., 0., ANGLES.z + LIM_X + delta_x * i as f64);
    let matrix_pitch = Matrix4::from_euler_angles(0., ANGLES.y + LIM_Y + delta_y * j as f64, 0.);
    
    Some(matrix_pitch * (matrix_yaw * VECTOR))
}

fn main() {
    println!("Vector: {}", VECTOR);
    // let a: Unit<Vector4<f64>> = Unit::new_normalize(Vector4::new(1.0, 0., 0., 0.).normalize());
    for r in (0..H).step_by(1) {
        for c in (0..W).step_by(1) {
            println!("({}, {}) -> {:.3?}", r, c, proj(c, r).unwrap());
        }
    }
    println!("{:.3}", proj(0, 0).unwrap());
    println!("{:.3}", proj(W / 2, H / 2).unwrap());
    println!("{:.3}", proj(0, H-1).unwrap());
    println!("{:.3}", proj(W-1, H-1).unwrap());
}
