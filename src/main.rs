mod renderer;

use image::{Rgb, RgbImage};

use renderer::objects::camera::{Dimensions, Camera, PerspectiveCamera};
use renderer::objects::ray::{Vector};
use renderer::objects::material::Material;
use renderer::objects::model::SphereModel;
use renderer::scene::Scene;
use renderer::{Renderer, SimpleRenderer};

fn main() {
    let dims = Dimensions{width: 800, height: 600};

    let cam = PerspectiveCamera::new(
      Vector::new(0., -10., 0., 0.),
      Vector::new(0., 0., 0., 0.),
      dims.clone(),
      std::f64::consts::FRAC_PI_6
    );

    let renderer = SimpleRenderer::new(Scene::new(vec![
        SphereModel::new(Vector::new(0., 0., 0., 0.), 1., Material::metallic()),
        SphereModel::new(Vector::new(1., -2., 0., 0.), 0.5, Material::marble())
    ]));

    let mut image = RgbImage::new(dims.width as u32, dims.height as u32);
    for j in 0..cam.get_dimensions().height{
        for i in 0..cam.get_dimensions().width{
            let ray = cam.gen_ray(i, j);
            let col = renderer.cast(&ray);
            image.put_pixel(i as u32, j as u32, Rgb::from([col[0], col[1], col[2]]));
        }
    }
    image.save("output.png").unwrap();
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

