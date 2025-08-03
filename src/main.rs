use image::RgbImage;
use crate::renderer::objects::camera::{Camera, Dimensions, PerspectiveCamera};
use crate::renderer::objects::material::{Material};
use crate::renderer::objects::model::TriangleModel;
use crate::renderer::objects::ray::Vector;
use crate::renderer::scene::Scene;
use crate::renderer::{Renderer, SimpleRenderer};

mod renderer;
mod tests;


fn main() {
    let cam = PerspectiveCamera::new(
        Vector::new(7.2, -4.2, 6.4, 0.),
        Vector::new(0., 0., 0., 0.),
        Dimensions {width: 800, height: 600},
        std::f64::consts::FRAC_PI_6,
    );

    let renderer = SimpleRenderer::new(Scene::new(vec![
        TriangleModel::from_stl(
            "Cup.stl",
            Material::marble()
        ).unwrap()
    ]));


    let dims = cam.get_dimensions();

    let mut image = RgbImage::new(dims.width as u32, dims.height as u32);
    for j in 0..dims.height {
        for i in 0..dims.width {
            let ray = cam.gen_ray(i, j);
            let col = renderer.cast(&ray);
            image.put_pixel(i as u32, j as u32, image::Rgb::from([col[0], col[1], col[2]]));
        }
    }

    image.save("artifacts/Cup.png").unwrap();
}



