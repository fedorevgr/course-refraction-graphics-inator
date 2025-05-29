use std::f64;
use nalgebra as na;
use nalgebra::Vector3;


const FOV_X: f64 = f64::consts::FRAC_PI_3 * 2.0;
const W: usize = 40;
const H: usize = 30;

const FOV_Y: f64 = FOV_X / W as f64 * H as f64;

const LIM_X: f64 = -FOV_X / 2.0;
const LIM_Y: f64 = -FOV_Y / 2.0;

const VECTOR: Vector3<f64> = Vector3::new(1.0, 0., 0.);


fn proj(i: usize, j: usize) -> Vector3<f64> {
    if (i > W) || (j > H) {
        panic!("Out of bound at index {}", i);
    }
    
    let delta_x = FOV_X / (W-1) as f64;
    let delta_y = FOV_Y / (H-1) as f64;
    
    let matrix_yaw = na::Rotation3::from_euler_angles(0., 0., LIM_X + delta_x * i as f64);
    let matrix_pitch = na::Rotation3::from_euler_angles(0., LIM_Y + delta_y * j as f64, 0.);
    
    matrix_pitch * (matrix_yaw * VECTOR)
}

fn main() {
    println!("{:.3?}", proj(0, 0));
    println!("{:.3?}", proj(0, H-1));
    println!("{:.3?}", proj(W-1, H-1));
}
