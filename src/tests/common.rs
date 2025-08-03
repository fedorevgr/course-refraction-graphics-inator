use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use image::RgbImage;

use crate::renderer::objects::camera::{Camera, Dimensions};
use crate::renderer::objects::material::{Material, MaterialBuilder, Rgb};
use crate::renderer::objects::model::SphereModel;
use crate::renderer::objects::ray::Vector;
use crate::renderer::Renderer;


pub struct Common {

}

impl Common {
    pub const DIMENSIONS: Dimensions = Dimensions { width: 100, height: 75 };
    const ARTIFACT_DIR: &'static str = "artifacts/";

    pub fn setup() {
        if !std::fs::create_dir(Self::ARTIFACT_DIR).is_err_and(|e| e.kind() == ErrorKind::AlreadyExists) {
            println!("Creating ARTIFACT_DIR");
        }
    }

    pub fn generate_image<P: AsRef<Path>, C: Camera, R: Renderer>(path: P, camera: &C, renderer: &R) {
        let dims = camera.get_dimensions();

        let mut image = RgbImage::new(dims.width as u32, dims.height as u32);
        for j in 0..dims.height {
            for i in 0..dims.width {
                let ray = camera.gen_ray(i, j);
                let col = renderer.cast(&ray);
                image.put_pixel(i as u32, j as u32, image::Rgb::from([col[0], col[1], col[2]]));
            }
        }

        image.save(PathBuf::from(Self::ARTIFACT_DIR).join(path)).unwrap();
    }

    pub fn get_3_spheres() -> Vec<SphereModel> {
        vec![
            SphereModel::new(Vector::new(0., 0., 0., 0.), 1., Material::metallic()),
            SphereModel::new(Vector::new(1., -2., 0., 0.), 0.5, Material::marble()),
            SphereModel::new(Vector::new(-1.3, 2., 0., 0.), 1.5,
                             MaterialBuilder::default()
                                 .color(Rgb::new(140, 200, 80))
                                 .metallic(Rgb::new(120, 120, 120))
                                 .roughness(Rgb::new(100, 100, 100))
                                 .k(4.).build().unwrap())
        ]
    }
}