use std::thread;
use std::thread::JoinHandle;
use image::RgbImage;
use crate::image_manager::{Color, Manager};
use crate::renderer::objects::camera::{Camera, Dimensions};
use crate::renderer::Renderer;

use crate::renderer::objects::ray::Rgb as RayRgb;

struct Block<C: Camera, R: Renderer> {
    size: usize,
    x: usize,
    y: usize,
    dimensions: Dimensions,
    camera: C,
    renderer: R,
}

impl<C: Camera + Send + 'static, R: Renderer + Send + 'static> Block<C, R> {
    pub fn new(
        size: usize,
        x: usize,
        y: usize,
        dimensions: Dimensions,
        camera: C,
        renderer: R,

    ) -> Self {
        Self {
            size,
            x,
            y,
            dimensions,
            camera,
            renderer
        }
    }

    pub fn run(self) -> JoinHandle<Vec<Color>> {
        thread::spawn(move || {
            let mut x = self.x;
            let mut y = self.y;

            let mut buffer: Vec<Color> = vec![Color::from([0; 3]); self.size];

            for i in 0..self.size {
                let ray = self.camera.gen_ray(x, y);
                let col = self.renderer.cast(&ray);

                buffer[i] = RayRgb(col).to_pixel().into();

                x += 1;
                if x == self.dimensions.width {
                    x = 0;
                    y += 1;
                }
            }
            buffer
        })
    }
}

pub struct MultiThread {
    pub thread_count: usize
}

impl MultiThread {
    fn generate_image<C: Camera + Clone + Send + 'static, R: Renderer + Clone + Send + 'static>(
        &self,
        camera: &C,
        renderer: &R,
    ) -> RgbImage {

        let dims = camera.get_dimensions();
        let total = dims.width * dims.height;

        let block_size = total / self.thread_count;
        let leftover = total % self.thread_count;

        let mut threads: Vec<_> = (0..self.thread_count)
            .map(|i| {
                Block::new(
                    block_size,
                    block_size * i % dims.width,
                    block_size * i / dims.width,
                    dims.clone(),
                    camera.clone(),
                    renderer.clone(),
                )
                    .run()
            })
            .collect();

        if leftover > 0 {
            threads.push(
                Block::new(
                    leftover,
                    block_size * self.thread_count % dims.width,
                    block_size * self.thread_count / dims.width,
                    dims.clone(),
                    camera.clone(),
                    renderer.clone(),
                )
                    .run(),
            );
        }

        let image_segments: Vec<Vec<Color>> =
            threads.into_iter().map(|h| h.join().unwrap()).collect();

        let _image: Vec<&Color> = image_segments
            .iter()
            .flat_map(|segment| segment.as_slice())
            .collect();

        RgbImage::from_raw(
            dims.width as u32,
            dims.height as u32,
            _image.iter().flat_map(|pix| pix.0).collect(),
        )
            .unwrap()
    }
}

impl<C, R> Manager<C, R> for MultiThread
where
    C: Camera + Clone + Send + 'static,
    R: Renderer + Clone + Send + 'static,
{
    fn create(&self, camera: &C, renderer: &R) -> RgbImage {
        self.generate_image(camera, renderer)
    }
}
