#![cfg(test)]
pub(crate) mod common;

use crate::renderer::objects::camera::perspective::PerspectiveCamera;
use crate::renderer::objects::ray::Vector;
use crate::renderer::objects::material::Rgb;
use crate::renderer::objects::model::triangle::TriangleModel;
use crate::renderer::scene::Scene;
use crate::renderer::implementations::simple_illumination::SimpleIllumination;
use crate::renderer::objects::material::MaterialBuilder;

use common::Common;
use crate::renderer::implementations::sampling::Sampling;

#[test]
fn test_simple_renderer_sphere_model() {
    Common::setup();
    let dims = Common::DIMENSIONS;

    let cam = PerspectiveCamera::new(
        Vector::new(0., -10., 0., 0.),
        Vector::new(0., 0., 0., 0.),
        dims.clone(),
        std::f64::consts::FRAC_PI_6
    );

    let renderer = Sampling::new(Scene::new(Common::get_3_spheres()));

    Common::generate_image("sphere_model.png", &cam, &renderer);
}

#[test]
fn test_simple_renderer_triangle_model() {
    Common::setup();
    let dims = Common::DIMENSIONS;

    let cam = PerspectiveCamera::new(
        Vector::new(0., -10., 0., 0.),
        Vector::new(0., 0., 0., 0.),
        dims.clone(),
        std::f64::consts::FRAC_PI_6,
    );

    let renderer = Sampling::new(Scene::new(vec![
        TriangleModel::from_stl(
            "../test_data/mesh.stl",
            MaterialBuilder::default()
                .color(Rgb::new(140, 200, 80))
                .metallic(Rgb::new(120, 120, 120))
                .roughness(Rgb::new(200, 200, 200))
                .k(4.).build().unwrap()
        ).unwrap()
    ]));

    Common::generate_image("triangle_model.png", &cam, &renderer);
}

#[test]
fn test_simple_renderer_cam_reposition() {
    Common::setup();
    let dims = Common::DIMENSIONS;

    let cam = PerspectiveCamera::new(
        Vector::new(10., -10., 10., 0.),
        Vector::new(0., 0., 0., 0.),
        dims.clone(),
        std::f64::consts::FRAC_PI_6
    );

    let renderer = SimpleIllumination::new(Scene::new(Common::get_3_spheres()));

    Common::generate_image("cam_reposition.png", &cam, &renderer);
}