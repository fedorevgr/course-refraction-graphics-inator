mod renderer;

use std::ops::Sub;

use nalgebra::Unit;
use renderer::*;
use crate::renderer::camera::{Camera, Dimensions};
use crate::renderer::model::Triangle;
use crate::renderer::ray::{Ray, Vector};
use image::{Rgb, RgbImage};

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

fn main() {
    let dims = Dimensions{width: 400, height: 300};

    let cam = Camera::new(
        Vector::new(0.0, 0.0, 0.0, 0.),
        0., 0.,
        std::f64::consts::PI,
        dims.clone()
    );
    println!("{:?}", cam);
    
    let mut image = RgbImage::new(dims.width as u32, dims.height as u32);
    let mut gen_ray = cam.pixel_vectors();
    for j in 0..cam.dimensions.height{
        for i in 0..cam.dimensions.width{
            image.put_pixel(i as u32, j as u32, hit_sphere(&gen_ray()));
        }
    }
    image.save("output.png").unwrap();
    return;
}
