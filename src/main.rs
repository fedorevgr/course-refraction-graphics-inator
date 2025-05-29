mod renderer;

use nalgebra::Unit;
use renderer::*;
use crate::renderer::camera::{Camera, Dimensions};
use crate::renderer::model::Triangle;
use crate::renderer::ray::Vector;

fn loading_model() {
    use std::fs::OpenOptions;
    let mut file = OpenOptions::new().read(true).open("mesh.stl").unwrap();
    let stl = stl_io::read_stl(&mut file).unwrap();
    let vertices: Vec<Vector> = stl.vertices.iter().map(|v| Vector::new(v[0] as f64, v[1] as f64, v[2] as f64, 0.)).collect();
    let faces: Vec<Triangle> = stl.faces.iter()
        .map(
            |poly| Triangle {
                normal: Vector::new(
                    poly.normal[0] as f64,
                    poly.normal[1] as f64,
                    poly.normal[2] as f64,
                    0.
                ),
                idx: poly.vertices
            } )
        .collect();

    let model = model::Model::new(vertices, faces);
    println!("{:#?}", model);
}

fn main() {
    let cam = Camera::new(
        Vector::new(0.0, 0.0, 0.0, 0.),
        Unit::new_normalize(Vector::new(0.0, 1.0, 0.0, 0.)),
        std::f64::consts::FRAC_PI_2,
        Dimensions{width: 3, height: 2}
    );
    println!("{:?}", cam);

    let mut gen_ray = cam.pixel_vectors();
    let mut buf: Vec<Vec<Vector>> = Vec::new();
    
    for j in 0..cam.dimensions.height{
        let mut row: Vec<Vector> = Vec::new();
        for i in 0..cam.dimensions.width{
            row.push(gen_ray());
            println!("{:.3?}", cam.get_vector(i, j))
        }
        buf.push(row);
    }
    println!("{:.3?}", buf);
    // println!("[0, 0] {:#?}", buf.first().unwrap().first().unwrap());
    // println!("[0, 1] {:#?}", buf.first().unwrap().last().unwrap());
    // println!("[1, 0] {:#?}", buf.last().unwrap().first().unwrap());
    // println!("[1, 1] {:#?}", buf.last().unwrap().last().unwrap());
}
