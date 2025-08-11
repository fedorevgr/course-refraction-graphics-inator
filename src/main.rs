mod renderer;
mod tests;
mod image_manager;

use rand::{SeedableRng};
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
            width: 400,
            height: 300,
        },
        std::f64::consts::FRAC_PI_6,
    );

    let scene = Scene::new(vec![
        SphereModel::new(
            Vector::from([0.; 4]),
            1.,
            MaterialBuilder::default()
                .color([0xFF, 0x20, 0xD0].into())
                .roughness([0xFF;3].into())
                .emissivity([0x40; 3].into())
                .metallic([0x0; 3].into())
                .build().unwrap()
        ),
        SphereModel::new(
            Vector::from([-2., 0., 0., 0.]),
            1.,
            MaterialBuilder::default()
                .color([0xFF, 0, 0].into())
                .roughness([0; 3].into())
                .emissivity([0xFF; 3].into())

                .build().unwrap()
        ),
        // TriangleModel::from_stl("test_data/PlateFront.stl", MaterialBuilder::default()
        //     .color([0xFF; 3].into())
        //     .roughness([0xFF;3].into())
        //     .emissivity([0x40; 3].into())
        //     .metallic([0x0; 3].into())
        //     .build().unwrap()).unwrap(),
        // TriangleModel::from_stl("test_data/PlateSun.stl", MaterialBuilder::default()
        //         .color([0xFF; 3].into())
        //         .roughness([0; 3].into())
        //         .emissivity([0xFF; 3].into())
        //         .build().unwrap()).unwrap(),
    ]
    );

    let renderer = Sampling::new(
        scene,
        Black{},
        4,
        rand_pcg::Pcg64Mcg::seed_from_u64(0),
    );

    // let manager = image_manager::implementations::rayon::Library::new(64 * 64);
    let manager = image_manager::implementations::one_thread::OneThreaded{};
    manager.create(&camera, &renderer).save("artifacts/Cube.png").unwrap();

}




