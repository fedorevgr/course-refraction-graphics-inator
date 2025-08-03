mod renderer;

use image::{RgbImage};

use renderer::objects::camera::{Dimensions, Camera, PerspectiveCamera};
use renderer::objects::ray::{Vector};
use renderer::objects::material::{Rgb};
use renderer::scene::Scene;
use renderer::{Renderer, SimpleRenderer};
use crate::renderer::objects::material::MaterialBuilder;
use crate::renderer::objects::model::TriangleModel;

fn main() {
    let dims = Dimensions { width: 80, height: 60 };

    let cam = PerspectiveCamera::new(
        Vector::new(0., -10., 0., 0.),
        Vector::new(0., 0., 0., 0.),
        dims.clone(),
        std::f64::consts::FRAC_PI_6,
    );

    let renderer = SimpleRenderer::new(Scene::new(vec![
        TriangleModel::from_stl(
            "mesh.stl",
            MaterialBuilder::default()
                .color(Rgb::new(140, 200, 80))
                .metallic(Rgb::new(200, 200, 200))
                .roughness(Rgb::new(200, 200, 200))
                .k(4.).build().unwrap()
        ).unwrap()
    ]));

    let mut image = RgbImage::new(dims.width as u32, dims.height as u32);
    for j in 0..cam.get_dimensions().height {
        for i in 0..cam.get_dimensions().width {
            let ray = cam.gen_ray(i, j);
            let col = renderer.cast(&ray);

            image.put_pixel(i as u32, j as u32, image::Rgb::from([col[0], col[1], col[2]]));
        }
    }
    image.save("output.png").unwrap();
}



