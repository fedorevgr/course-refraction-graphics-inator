mod renderer;
use renderer::*;
use crate::renderer::model::Triangle;
use crate::renderer::ray::Vector;

fn main() {

    use std::fs::OpenOptions;
    let mut file = OpenOptions::new().read(true).open("mesh.stl").unwrap();
    
    let stl = stl_io::read_stl(&mut file).unwrap();
    

    let vertices: Vec<Vector> = stl.vertices.iter().map(|v| Vector::new(v[0] as f64, v[1] as f64, v[2] as f64)).collect();
    let faces: Vec<Triangle> = stl.faces.iter()
        .map(
            |poly| Triangle { 
                normal: Vector::new(
                    poly.normal[0] as f64, 
                    poly.normal[1] as f64, 
                    poly.normal[2] as f64
                ), 
                idx: poly.vertices
            } )
        .collect();
    
    let model = model::Model::new(vertices, faces);
    println!("{:#?}", model);
}
