use std::fs::File;
use std::io::Write;
use rand::{rng};
use rand_distr::{Distribution, Normal};

use nalgebra::{Vector4};


const N: u32 = 10;
const DIFF: f64 = 0.2;


fn gen_diffusion(v: &Vector4<f64>) -> Vector4<f64> {
    let mut rng = rng();
    let normal = Normal::new(0.0, DIFF).unwrap();
    let sin = normal.sample(&mut rng).sin();
    let cos = (1. - sin * sin).sqrt();

    Vector4::new(v.x * cos - v.y * sin, v.x * sin + v.y * cos, v.z, v.w)
}


fn main() {
    let normal = Vector4::new(0., 1., 0., 0.).normalize();
    let vector = Vector4::new(1., -1., 0., 0.).normalize();
    let reflected_ideal: Vector4<f64> =  vector - 2. * (vector.dot(&normal)) * normal;

    let reflected: Vec<Vector4<f64>> = (0..N)
        .map(|_| gen_diffusion(&reflected_ideal))
        .collect();

    let mut f = File::create("data.txt").unwrap();
    f.write_fmt(format_args!("{} {}\n", reflected_ideal.x, reflected_ideal.y)).unwrap();
    for ray in reflected {
        f.write_fmt(format_args!("{} {}\n", ray.x, ray.y)).unwrap();
    }
}
