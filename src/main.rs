use crate::renderer::objects::camera::{Camera, Dimensions, PerspectiveCamera};
use crate::renderer::objects::material::Material;
use crate::renderer::objects::model::SphereModel;
use crate::renderer::objects::ray::Vector;
use crate::renderer::scene::Scene;
use crate::renderer::SimpleRenderer;

mod renderer;
mod tests;


fn main() {
    tests::common::Common::setup();
    let dims = Dimensions {width: 40, height: 30};

    let cam = PerspectiveCamera::new(
        Vector::new(5., -5.0, 5., 0.),
        Vector::new(0., 0., 0., 0.),
        dims.clone(),
        std::f64::consts::FRAC_PI_6
    );

    let renderer = SimpleRenderer::new(Scene::new(
        vec![SphereModel::new(Vector::new(0., 0., 0., 0.), 1., Material::marble())]
    ));

    println!("{:.3?}", cam.gen_ray(dims.width / 2, dims.height / 2));

    tests::common::Common::generate_image("output.png", &cam, &renderer);
}



