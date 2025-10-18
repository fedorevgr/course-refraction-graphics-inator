use image::RgbImage;
use crate::image_generator::{Color, ImageGenerator};
use crate::renderer::objects::camera::Camera;
use crate::renderer::Renderer;
use crate::renderer::objects::ray::Rgb as RayRgb;

pub struct OneThreaded {
}

impl OneThreaded {
    fn generate_image<C: Camera, R: Renderer>(
        &self,
        camera: &C,
        renderer: &R,
    ) -> RgbImage {
        let dims = camera.get_dimensions();

        RgbImage::from_fn(dims.width as u32, dims.height as u32, |x, y| {
            let ray = camera.gen_ray(x as usize, y as usize);
            let col = renderer.cast(&ray);
            RayRgb(col).to_pixel().into()
        })
    }
}

impl<C, R> ImageGenerator<C, R> for OneThreaded
where
    C: Camera,
    R: Renderer,
{
    fn create(&self, camera: &C, renderer: &R) -> RgbImage {
        self.generate_image(camera, renderer)
    }
}
