pub mod implementations;

use image::RgbImage;
use crate::renderer::objects::camera::Camera;
use crate::renderer::Renderer;

pub type Color = image::Rgb<u8>;

pub trait ImageGenerator<C: Camera, R: Renderer> {
    fn create(&self, camera: &C, renderer: &R) -> RgbImage;
}
