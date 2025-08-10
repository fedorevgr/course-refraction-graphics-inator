mod renderer;

use crate::renderer::objects::camera::{Dimensions, PerspectiveCamera};
use crate::renderer::objects::material::{Material, Rgb};
use crate::renderer::objects::model::TriangleModel;
use crate::renderer::objects::ray::Vector;
use crate::renderer::scene::Scene;
use crate::renderer::SimpleRenderer;
use crate::image_manager::Manager;

mod tests;
mod image_manager;



fn main() {

    let camera = PerspectiveCamera::new(
        Vector::new(7.2, -4.2, 6.4, 0.),
        Vector::new(0., 0., 0., 0.),
        Dimensions {
            width: 1920,
            height: 1080,
        },
        std::f64::consts::FRAC_PI_6,
    );

    let renderer = SimpleRenderer::new(Scene::new(vec![
        TriangleModel::from_stl("test_data/Cube.stl", {
            let mut m = Material::metallic();
            m.roughness = Rgb::from([20, 20, 20]);
            m.k = 3.;
            m
        })
        .unwrap(),
    ]));


    let manager = image_manager::Library::new(64 * 64);
    manager.create(&camera, &renderer).save("artifacts/Cube.png").unwrap();
}




