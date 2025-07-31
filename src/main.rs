mod renderer;

use std::ops::Sub;

use renderer::*;
use renderer::objects::camera::{Camera, Dimensions};
use renderer::objects::ray::{Ray, Vector};
use image::{Rgb, RgbImage};

const SPHERE_POS: Vector = Vector::new(2., 0., 0., 0.);
const SPHERE_R_2: f64 = 0.5 * 0.5;

const SPHERE_COL: Rgb<u8> = Rgb([255, 0, 0]);

fn hit_sphere(ray: &Ray) -> Rgb<u8>
{
    let a = ray.direction.magnitude();
    let b = 2. * ray.direction.dot(&SPHERE_POS.sub(ray.origin));
    let c  = SPHERE_POS.sub(ray.origin).magnitude() - SPHERE_R_2;

    if b * b - 4. * a * c < 0. {
        Rgb([0, 0, 0])
    }
    else {
        SPHERE_COL
    }
}

const TORUS_R: f64 = 1. * 1.;
const TORUS_K: f64 = 0.5 * 0.5;

fn hit_torus(ray: &Ray) -> Rgb<u8>
{
    let step = 0.05;
    let dir = ray.direction.normalize();
    for i in 0..60 {
        let t = step * i as f64;
        let p = ray.origin + dir * t;
        
        if (p.magnitude_squared() + TORUS_R - TORUS_K).powi(2) < 4. * TORUS_R * (p.x.powi(2) + p.y.powi(2))
        {
            let norm = (p - (p - Vector::new(0., 0., p.z, 0.)).normalize()).dot(&p);
            return Rgb([(155. * norm) as u8 + 100, 0, 0]);
        }
    }
    Rgb([0, 0, 0])
}

fn main() {
    let dims = Dimensions{width: 400, height: 300};

    let cam = Camera::new(
        Vector::new(-0., 0., -0.0, 0.),
        0., -1.,
        std::f64::consts::PI * 4.,
        dims.clone()
    );
    println!("{:?}", cam);
    
    let mut image = RgbImage::new(dims.width as u32, dims.height as u32);
    let mut gen_ray = cam.pixel_vectors();
    for j in 0..cam.dimensions.height{
        for i in 0..cam.dimensions.width{
            image.put_pixel(i as u32, j as u32, hit_torus(&gen_ray()));
        }
    }
    image.save("output.png").unwrap();
    return;
}


// fn loading_model() {
//     use std::fs::OpenOptions;
//     let mut file = OpenOptions::new().read(true).open("mesh.stl").unwrap();
//     let stl = stl_io::read_stl(&mut file).unwrap();
//     let vertices: Vec<Vector> = stl.vertices.iter().map(|v| Vector::new(v[0] as f64, v[1] as f64, v[2] as f64, 0.)).collect();
//     let faces: Vec<Triangle> = stl.faces.iter()
//         .map(
//             |poly| Triangle {
//                 normal: Vector::new(
//                     poly.normal[0] as f64,
//                     poly.normal[1] as f64,
//                     poly.normal[2] as f64,
//                     0.
//                 ),
//                 idx: poly.vertices
//             } )
//         .collect();
//
//     let model = model::Model::new(vertices, faces);
//     println!("{:#?}", model);
// }

