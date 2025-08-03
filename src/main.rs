use image::{RgbImage};

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
    let camera = PerspectiveCamera::new(
        Vector::new(7.2, -4.2, 6.4, 0.),
        Vector::new(0., 0., 0., 0.),
        Dimensions {width: 400, height: 300},
        std::f64::consts::FRAC_PI_6,
    );

    let renderer = SimpleRenderer::new(Scene::new(vec![
        TriangleModel::from_stl(
            "Cup.stl",
            {
                let mut m = Material::metallic();
                m.roughness = Rgb::from([20, 20, 20]);
                m.k = 3.;
                m
            }
        ).unwrap()
    ]));

    generate_image(&camera, &renderer, 2).save("artifacts/Cup.png").unwrap();
}

struct Block<C: Camera, R: Renderer> {
    buffer: Vec<Color>,
    x: usize,
    y: usize,
    dimensions: Dimensions,
    camera: C,
    renderer: R,
}

impl<C: Camera, R: Renderer> Block<C, R> {
    pub fn new(size: usize, x: usize, y: usize, dimensions: Dimensions, camera: C, renderer: R) -> Self {
        Self {
            buffer: vec![Color::from([0; 3]); size],
            x,
            y,
            dimensions,
            camera,
            renderer
        }
    }

    pub fn run(&mut self) {
        let mut x = self.x;
        let mut y = self.y;
        for i in 0..self.buffer.len() {

            let ray = self.camera.gen_ray(x, y);
            let col = self.renderer.cast(&ray);

            self.buffer[i] = Color::from([col[0], col[1], col[2]]);

            x += 1;
            if x == self.dimensions.width {
                x = 0;
                y += 1;
            }
        }
    }
}

fn generate_image<C: Camera + Clone, R: Renderer + Clone>(
    camera: &C, renderer: &R, threads: usize
) -> RgbImage {
    let dims = camera.get_dimensions();
    let total = dims.width * dims.height;

    let block_size = total / threads;
    let leftover = total % threads;

    let mut block_placements: Vec<_> = (0..threads).map(|i|
        Block::new(block_size, block_size * i % dims.width, block_size * i / dims.width, dims.clone(), camera.clone(), renderer.clone())
    ).collect();
    if leftover > 0 {
        block_placements.push(Block::new(leftover, block_size * threads % dims.width, block_size * threads / dims.width, dims.clone(), camera.clone(), renderer.clone()));
    }

    block_placements.iter_mut().for_each(Block::run);

    let _image: Vec<&Color> = block_placements.iter().flat_map(|block| block.buffer.as_slice()).collect();

    RgbImage::from_raw(dims.width as u32, dims.height as u32, _image.iter().flat_map(|pix| pix.0).collect()).unwrap()
}

