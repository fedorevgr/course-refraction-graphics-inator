mod renderer;
mod tests;
mod image_manager;

use image::{Rgb, RgbImage};
use rand::{Rng, SeedableRng};
use crate::renderer::objects::camera::Dimensions;
use crate::renderer::objects::camera::perspective::PerspectiveCamera;
use crate::renderer::objects::material::MaterialBuilder;
use crate::renderer::objects::ray::Vector;
use crate::renderer::scene::Scene;
use crate::renderer::implementations::sampling::{Sampling, Black};
use crate::image_manager::Manager;
use crate::renderer::objects::model::sphere::SphereModel;

fn main() {
    let camera = PerspectiveCamera::new(
        Vector::new(0., -10., 0., 0.),
        Vector::new(0., 0., 0., 0.),
        Dimensions {
            width: 60,
            height: 40,
        },
        std::f64::consts::FRAC_PI_6 / 1.,
    );

    let scene = Scene::new(vec![
        SphereModel::new(
            Vector::from([0.; 4]),
            1.,
            MaterialBuilder::default()
                .color([20; 3].into())
                .roughness([0xFF;3].into())
                .emissivity([0x10; 3].into())
                .build().unwrap()
        ),
        SphereModel::new(
            Vector::from([-2., 0., 0., 0.]),
            1.,
            MaterialBuilder::default()
                .color([0; 3].into())
                .roughness([0; 3].into())
                .emissivity([0xFF; 3].into())
                .build().unwrap()
        ),
    ]
    );

    let renderer = Sampling::new(
        scene,
        Black{},
        5,
        rand_chacha::ChaCha8Rng::seed_from_u64(0),
    );

    // let manager = image_manager::implementations::rayon::Library::new(64 * 64);
    let manager = image_manager::implementations::one_thread::OneThreaded{};
    manager.create(&camera, &renderer).save("artifacts/Cube.png").unwrap();

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(1);
    RgbImage::from_fn(1920, 1080, |_, _| Rgb([rng.random(); 3])).save("artifacts/Noise.png").unwrap();
}




