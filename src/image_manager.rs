use std::thread;
use std::thread::JoinHandle;

use crate::renderer::Renderer;
use crate::renderer::objects::camera::{Camera, Dimensions};

use image::RgbImage;
use indicatif::{MultiProgress, ProgressBar};

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

    pub fn run(self, pb: ProgressBar) -> JoinHandle<Vec<Color>> {
        thread::spawn(move || {
            let mut x = self.x;
            let mut y = self.y;

            let mut buffer: Vec<Color> = vec![Color::from([0; 3]); self.size];

            for i in 0..self.size {
                pb.tick();

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
    pub fn new(thread_count: usize, show_status: bool) -> Self {
        Self {
            thread_count
        }
    }

    fn generate_image<C: Camera + Clone + Send + 'static, R: Renderer + Clone + Send + 'static>(
        &self,
        camera: &C,
        renderer: &R,
    ) -> RgbImage {
        let progress_bar = MultiProgress::new();

        let dims = camera.get_dimensions();
        let total = dims.width * dims.height;

        let block_size = total / self.thread_count;
        let leftover = total % self.thread_count;

        let mut threads: Vec<_> = (0..self.thread_count)
            .map(|i| {
                let pb = progress_bar.add(ProgressBar::new(block_size as u64));
                Block::new(
                    block_size,
                    block_size * i % dims.width,
                    block_size * i / dims.width,
                    dims.clone(),
                    camera.clone(),
                    renderer.clone(),
                )
                .run(pb)
            })
            .collect();

        if leftover > 0 {
            let pb = progress_bar.add(ProgressBar::new(leftover as u64));
            threads.push(
                Block::new(
                    leftover,
                    block_size * self.thread_count % dims.width,
                    block_size * self.thread_count / dims.width,
                    dims.clone(),
                    camera.clone(),
                    renderer.clone(),
                )
                .run(pb),
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
