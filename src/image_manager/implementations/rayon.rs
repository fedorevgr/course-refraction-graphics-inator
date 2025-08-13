#![allow(dead_code)]

use image::RgbImage;

use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;

use crate::image_manager::{Color, Manager};
use crate::renderer::objects::camera::Camera;
use crate::renderer::Renderer;

pub struct Library {
    size: usize,
}
impl Library {
    pub fn new(size: usize) -> Library {
        Library { size }
    }

    fn generate_image<C: Camera + Clone + Send + 'static + Sync, R: Renderer + Clone + Send + 'static + Sync>(
        &self,
        camera: &C,
        renderer: &R,
    ) -> RgbImage {
        let dims = camera.get_dimensions();
        let block_count = dims.width * dims.height / self.size;
        let leftover = (dims.width * dims.height) % self.size;

        let mut blocks: Vec<[usize; 4]> = (0..block_count)
            .map(|i| {
                [self.size, self.size * i  % dims.width, self.size * i / dims.width, dims.width]
            })
            .collect();
        blocks.push([leftover, self.size * block_count % dims.width, self.size * block_count / dims.width, dims.width]);

        let pixels: Vec<Color> = blocks.par_iter().flat_map(|&[_size, _x, _y, _width]| {
            let mut x = _x;
            let mut y = _y;

            let mut buffer: Vec<Color> = vec![Color::from([0; 3]); self.size];

            for i in 0.._size {
                let ray = camera.gen_ray(x, y);
                let col = renderer.cast(&ray);

                buffer[i] = Color::from([col[0], col[1], col[2]]);

                x += 1;
                if x == _width {
                    x = 0;
                    y += 1;
                }
            }
            buffer
        }).collect();

        RgbImage::from_raw(dims.width as u32, dims.height as u32, pixels.iter().flat_map(|pix | pix.0).collect()).unwrap()
    }
}

impl<C, R> Manager<C, R> for Library
where
    C: Camera + Clone + Send + 'static + Sync,
    R: Renderer + Clone + Send + 'static + Sync,
{
    fn create(&self, camera: &C, renderer: &R) -> RgbImage {
        self.generate_image(camera, renderer)
    }
}
