use image::RgbImage;
use std::thread;
use std::thread::JoinHandle;
use crate::renderer::objects::camera::{Camera, Dimensions, PerspectiveCamera};
use crate::renderer::objects::material::{Material, Rgb};
use crate::renderer::objects::model::TriangleModel;
use crate::renderer::objects::ray::Vector;
use crate::renderer::scene::Scene;
use crate::renderer::{Renderer, SimpleRenderer};

mod renderer;
mod tests;

type Color = image::Rgb<u8>;

fn main() {
    let start = std::time::SystemTime::now();
    let camera = PerspectiveCamera::new(
        Vector::new(7.2, -4.2, 6.4, 0.),
        Vector::new(0., 0., 0., 0.),
        Dimensions {
            width: 400,
            height: 300,
        },
        std::f64::consts::FRAC_PI_6,
    );

    let renderer = SimpleRenderer::new(Scene::new(vec![
        TriangleModel::from_stl("Cup.stl", {
            let mut m = Material::metallic();
            m.roughness = Rgb::from([20, 20, 20]);
            m.k = 3.;
            m
        })
        .unwrap(),
    ]));

    generate_image(&camera, &renderer, 10)
        .save("artifacts/Cup.png")
        .unwrap();
    println!("Elapsed time: {:?}", start.elapsed().unwrap());
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
            renderer,
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

fn generate_image<C: Camera + Clone + Send + 'static, R: Renderer + Clone + Send + 'static>(
    camera: &C,
    renderer: &R,
    thread_count: usize,
) -> RgbImage {
    let dims = camera.get_dimensions();
    let total = dims.width * dims.height;

    let block_size = total / thread_count;
    let leftover = total % thread_count;

    let threads: Vec<_> = (0..thread_count)
        .map(|i| {
            Block::new(
                block_size,
                block_size * i % dims.width,
                block_size * i / dims.width,
                dims.clone(),
                camera.clone(),
                renderer.clone(),
            ).run()
        })
        .collect();

    let mut image_segments: Vec<Vec<Color>> = threads.into_iter().map(|h| h.join().unwrap()).collect();

    if leftover > 0 {
        image_segments.push(
            Block::new(
                leftover,
                block_size * thread_count % dims.width,
                block_size * thread_count / dims.width,
                dims.clone(),
                camera.clone(),
                renderer.clone(),
            )
            .run().join().unwrap(),
        );
    }


    let _image: Vec<&Color> = image_segments
        .iter()
        .flat_map(|segment| segment.as_slice())
        .collect();

    RgbImage::from_raw(dims.width as u32, dims.height as u32, _image.iter().flat_map(|pix| pix.0).collect()).unwrap()
}
