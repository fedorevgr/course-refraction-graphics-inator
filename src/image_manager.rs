use std::thread;
use std::thread::JoinHandle;

use crate::renderer::Renderer;
use crate::renderer::objects::camera::{Camera, Dimensions};

use image::RgbImage;
use rayon::prelude::*;

type Color = image::Rgb<u8>;

pub trait Manager<C: Camera, R: Renderer> {
    fn create(&self, camera: &C, renderer: &R) -> RgbImage;
}

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
            Color::from([col[0], col[1], col[2]])
        })
    }
}

impl<C, R> Manager<C, R> for OneThreaded
where
    C: Camera,
    R: Renderer,
{
    fn create(&self, camera: &C, renderer: &R) -> RgbImage {
        self.generate_image(camera, renderer)
    }
}

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

                buffer[i] = Color::from([col[0], col[1], col[2]]);

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
