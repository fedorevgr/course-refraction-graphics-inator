mod renderer;
mod tests;
mod image_manager;


// use rand::{SeedableRng};
use crate::renderer::objects::camera::Dimensions;
use crate::renderer::objects::camera::perspective::PerspectiveCamera;
use crate::renderer::objects::material::MaterialBuilder;
use crate::renderer::objects::ray::Vector;
use crate::renderer::scene::Scene;
// use crate::renderer::implementations::sampling::{Sampling, Black};
use crate::image_manager::Manager;
use crate::renderer::implementations::global_illumination::{GlobalIllumination, PointLight, Solid, WithSky};
use crate::renderer::implementations::sampling::Black;
use crate::renderer::objects::model::sphere::SphereModel;

fn main() {
    let camera = PerspectiveCamera::new(
        Vector::new(-3., -13.4, 3., 0.),
        Vector::new(0., 0., 0., 0.),
        Dimensions {
            width: 1200,
            height: 800,
        },
        std::f64::consts::FRAC_PI_6,
    );

    let scene = Scene::new(vec![
        // SphereModel::new(
        //     Vector::from([0.; 4]),
        //     1.,
        //     MaterialBuilder::default()
        //         .color([0., 1., 0.].into())
        //         .roughness([1.; 3].into())
        //         .metallic([0.; 3].into())
        //         .build().unwrap()
        // ),
        // SphereModel::new(
        //     Vector::from([-2., 0., 0., 0.]),
        //     1.,
        //     MaterialBuilder::default()
        //         .color([0., 0., 1.].into())
        //         .roughness([1.; 3].into())
        //         .metallic([0.; 3].into())
        //         .build().unwrap()
        // ),
        SphereModel::new(
            Vector::from([0., 0., -30.19, 0.]),
            59. / 2.,
            MaterialBuilder::default()
                .color([0.5; 3].into())
                .roughness([1.; 3].into())
                .metallic([0.; 3].into())
                .build().unwrap()
        )
    ]
    );

    let renderer = GlobalIllumination::new(
        scene,
        vec![PointLight::new([0., 0., 4., 0.].into(), 2., [1., 1., 1.].into()),],
        3,
        Solid::new([0.; 3].into()),
    );

    // let manager = image_manager::implementations::rayon::Library::new(64 * 64);

    let manager = image_manager::implementations::one_thread::OneThreaded{};
    manager.create(&camera, &renderer).save("artifacts/Spheres.png").unwrap();

}




