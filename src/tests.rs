use image::RgbImage;

use crate::renderer::objects::camera::{Dimensions, Camera, PerspectiveCamera};
use crate::renderer::objects::ray::{Vector};
use crate::renderer::objects::material::{Material, Rgb};
use crate::renderer::objects::model::SphereModel;
use crate::renderer::scene::Scene;
use crate::renderer::{Renderer, SimpleRenderer};
use crate::renderer::objects::material::MaterialBuilder;

#[test]
fn test_simple_renderer() {
    let dims = Dimensions{width: 800, height: 600};

    let cam = PerspectiveCamera::new(
        Vector::new(0., -10., 0., 0.),
        Vector::new(0., 0., 0., 0.),
        dims.clone(),
        std::f64::consts::FRAC_PI_6
    );

    let renderer = SimpleRenderer::new(Scene::new(vec![
        SphereModel::new(Vector::new(0., 0., 0., 0.), 1., Material::metallic()),
        SphereModel::new(Vector::new(1., -2., 0., 0.), 0.5, Material::marble()),
        SphereModel::new(Vector::new(-1.3, 2., 0., 0.), 1.5,
                         MaterialBuilder::default()
                             .color(Rgb::new(140, 200, 80))
                             .metallic(Rgb::new(120, 120, 120))
                             .roughness(Rgb::new(100, 100, 100))
                             .k(4.).build().unwrap())
    ]));

    let mut image = RgbImage::new(dims.width as u32, dims.height as u32);
    for j in 0..cam.get_dimensions().height{
        for i in 0..cam.get_dimensions().width{
            let ray = cam.gen_ray(i, j);
            let col = renderer.cast(&ray);
            image.put_pixel(i as u32, j as u32, image::Rgb::from([col[0], col[1], col[2]]));
        }
    }
    image.save("output.png").unwrap();
}