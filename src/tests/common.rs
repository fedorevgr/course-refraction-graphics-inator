use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::image_manager::Manager;
use crate::image_manager::implementations::one_thread::OneThreaded;

use crate::renderer::Renderer;

use crate::renderer::objects::camera::{Camera, Dimensions};
use crate::renderer::objects::material::{Material, MaterialBuilder, Rgb};
use crate::renderer::objects::model::sphere::SphereModel;
use crate::renderer::objects::ray::Vector;


pub struct Common {}

impl Common {
    pub const DIMENSIONS: Dimensions = Dimensions {
        width: 100,
        height: 75,
    };
    const ARTIFACT_DIR: &'static str = "artifacts/";

    pub fn setup() {
        if !std::fs::create_dir(Self::ARTIFACT_DIR)
            .is_err_and(|e| e.kind() == ErrorKind::AlreadyExists)
        {
            println!("Creating ARTIFACT_DIR");
        }
    }

    pub fn generate_image<P: AsRef<Path>, C: Camera, R: Renderer>(
        path: P,
        camera: &C,
        renderer: &R,
    ) {
        OneThreaded {}.create(camera, renderer)
        .save(PathBuf::from(Self::ARTIFACT_DIR).join(path))
        .unwrap();
    }

    pub fn get_3_spheres() -> Vec<SphereModel> {
        vec![
            SphereModel::new(Vector::new(0., 0., 0., 0.), 1., Material::metallic()),
            SphereModel::new(Vector::new(1., -2., 0., 0.), 0.5, Material::marble()),
            SphereModel::new(
                Vector::new(-1.3, 2., 0., 0.),
                1.5,
                MaterialBuilder::default()
                    .color(Rgb::new(140, 200, 80))
                    .metallic(Rgb::new(120, 120, 120))
                    .roughness(Rgb::new(100, 100, 100))
                    .k(4.)
                    .build()
                    .unwrap(),
            ),
        ]
    }
}
