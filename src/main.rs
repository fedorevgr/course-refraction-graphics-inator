use crate::renderer::objects::camera::{Dimensions, PerspectiveCamera};
use crate::renderer::objects::material::{Material, Rgb};
use crate::renderer::objects::model::TriangleModel;
use crate::renderer::objects::ray::Vector;
use crate::renderer::scene::Scene;
use crate::renderer::SimpleRenderer;
use crate::image_manager::Manager;

mod renderer;
mod tests;
mod image_manager;



fn main() {
    let start = std::time::SystemTime::now();
    let camera = PerspectiveCamera::new(
        Vector::new(7.2, -4.2, 6.4, 0.),
        Vector::new(0., 0., 0., 0.),
        Dimensions {
            width: 400,
            height: 300,
        },
        std::f64::consts::FRAC_PI_6,
    );

    let renderer = SimpleRenderer::new(Scene::new(vec![
        TriangleModel::from_stl("Cup.stl", {
            let mut m = Material::metallic();
            m.roughness = Rgb::from([20, 20, 20]);
            m.k = 3.;
            m
        })
        .unwrap(),
    ]));

    let i_man = image_manager::MultiThread { thread_count: 5};
    i_man.create(&camera, &renderer)
        .save("artifacts/Cup.png")
        .unwrap();
    println!("Elapsed time: {:?}", start.elapsed().unwrap());
}




